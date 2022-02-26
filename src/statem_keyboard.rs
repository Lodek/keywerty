use crate::keyboard::{Event, KeyId, Action, Keyboard};
use crate::keyboard::map::{KeyConf, TapKeyConf, HoldKeyConf, DoubleTapKeyConf, DoubleTapHoldKeyConf, KeyActionSet, KeyAction, LayerMap};

// TODO another detail i need to take care of is how the state machine will interact when two stateful
// keys are pressed.
// say key 1 and 2 are stateful
// what happens if i'm handling the stateM for key 1 and then key 2 is pressed?
// need to address this case to make everything works accordingly.
// 
// Once the StateM returns a `Some` it means it's dead
// with that said, the StateM is only going to handle they key its associated to
// Suppose key 32 is stateful, the stateM is only outputting an action for key 32 
// and not any other key in the keyboard.
// This way, once two stateful actions happen, first the current state machine will be handled
// it returns an action and then the new stateM can be spawned and handled

/// Some key activation modes are stateful in nature and depends
/// on the state of the other keys in the keyboard to perform an action.
/// The KeyboardStateM is a trait that can be implemented for stateful
/// activation modes.
///
/// Each time a stateful key is pressed, a new state machine should be created
/// to handle that state.
/// Once the key has performed an action, the state machine will be in one of
/// its activation states and should be discarded.
trait KeyStateMachine {
    type KeyConf;

    /// Steps the state machine from the current events
    /// The state machine will either return a KeyActionSet or None.
    /// Once a state machine returns an KeyActionSet it has reached one 
    /// of its accepting states and should be discarded.
    fn transition<'a>(&mut self, events: &'a [Event]) -> Option<KeyActionSet>;

    /// Initialize a State Machine instance.
    /// key_id indicates the key to which the state machine is attached.
    /// key_conf is the set of actions the key shall perform
    fn init_machine(&mut self, key_id: KeyId, key_conf: Self::KeyConf);
}

/// Module for Key State Machine implementation for the `Hold` key configuration
mod HoldKeyStateMachine {

    use super::*;
    use std::time::{Instant, Duration};

    #[derive(Clone, Copy, Debug)]
    enum State {
        Waiting,
        Hold,
        Tap
    }

    #[derive(Debug)]
    pub struct HoldKSM {
        watched_key: KeyId,
        state: State,
        key_conf: HoldKeyConf,
        creation: Instant,
        release_delay: Duration,
        initialized: bool,
    }

    impl HoldKSM {
        pub fn new(release_delay: Duration) -> Self {
            return Self {
                creation: Instant::now(),
                release_delay,
                state: State::Waiting,
                key_conf: HoldKeyConf::default(),
                watched_key: KeyId::default(),
                initialized: false,
            }
        }
    }

    impl KeyStateMachine for HoldKSM {
        type KeyConf = HoldKeyConf;

        fn init_machine(&mut self, key_id: KeyId, key_conf: HoldKeyConf) {
            self.watched_key = key_id;
            self.key_conf = key_conf;
            self.initialized = true;
            self.creation = Instant::now();
        }

        fn transition<'a>(&mut self, events: &'a [Event]) -> Option<KeyActionSet> {

            if let State::Waiting = self.state {
                if (Instant::now() - self.creation) > self.release_delay {
                    self.state = State::Hold;
                }

                else if events.contains(&Event::KeyRelease(self.watched_key)) {
                    self.state = State::Tap;
                }

                else if events.iter().any(|event| event.is_key_press()) {
                    self.state = State::Hold;
                }
            }

            match self.state {
                State::Waiting => None,
                State::Tap => Some(self.key_conf.0),
                State::Hold => Some(self.key_conf.1)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::time::Duration;
        use std::thread::sleep;

        #[test]
        fn test_kms_hold_till_timeout() {
            // Given Hold state machine watching key 10
            // And key conf for key 43 on tap and 42 on hold
            let tap_key_code = 43;
            let watched_key = 10;
            let hold_key_code = 42;
            let timeout = Duration::from_nanos(1);
            let tap_action = KeyActionSet::Single(KeyAction::AddKey(tap_key_code));
            let hold_action = KeyActionSet::Single(KeyAction::AddKey(hold_key_code));
            let conf = HoldKeyConf(tap_action, hold_action);
            let mut machine = HoldKSM::new(timeout);
            machine.init_machine(watched_key, conf);

            // Given key is held for longer than timeout
            sleep(timeout);

            // When machine is transitioned 
            let actionset_opt = machine.transition(&vec![]);

            // Then Optional contains actionset
            // And action returned hold key code
            assert_eq!(actionset_opt.unwrap(), KeyActionSet::Single(KeyAction::AddKey(hold_key_code)));
        }

        #[test]
        fn test_kms_other_key_press() {
            // Given Hold state machine watching key 10
            // And key conf for key 43 on tap and 42 on hold
            let watched_key = 10;
            let tap_key_code = 43;
            let hold_key_code = 42;
            let timeout = Duration::from_millis(1);
            let tap_action = KeyActionSet::Single(KeyAction::AddKey(tap_key_code));
            let hold_action = KeyActionSet::Single(KeyAction::AddKey(hold_key_code));
            let conf = HoldKeyConf(tap_action, hold_action);
            let mut machine = HoldKSM::new(timeout);
            machine.init_machine(watched_key, conf);

            // When machine is transitioned 
            let actionset_opt = machine.transition(&vec![Event::KeyPress(222)]);

            // Then Optional contains actionset
            // And action returned hold key code
            assert_eq!(actionset_opt.unwrap(), KeyActionSet::Single(KeyAction::AddKey(hold_key_code)));
        }

        #[test]
        fn test_kms_release() {
            // Given Hold state machine watching key 10
            // And key conf for key 43 on tap and 42 on hold
            let watched_key = 10;
            let tap_key_code = 43;
            let hold_key_code = 42;
            let timeout = Duration::from_millis(1);
            let tap_action = KeyActionSet::Single(KeyAction::AddKey(tap_key_code));
            let hold_action = KeyActionSet::Single(KeyAction::AddKey(hold_key_code));
            let conf = HoldKeyConf(tap_action, hold_action);
            let mut machine = HoldKSM::new(timeout);
            machine.init_machine(watched_key, conf);

            // When the watched key is released
            let actionset_opt = machine.transition(&vec![Event::KeyRelease(watched_key)]);

            // Then Optional contains actionset
            // And action returned hold key code
            assert_eq!(actionset_opt.unwrap(), KeyActionSet::Single(KeyAction::AddKey(tap_key_code)));
        }
    }
}


mod double_tap_key_state_machine {

    use super::*;
    use std::time::{Duration, Instant};

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum State {
        FirstTap,
        FirstRelease,
        Retap,
        Timeout
    }

    #[derive(Debug)]
    pub struct DoubleTapKSM {
        state: State,
        retap_threshold: Duration,
        hold_threshold: Duration,

        watched_key: KeyId,
        key_conf: DoubleTapKeyConf,
        creation: Instant,
        initialized: bool,
        release_timestamp: Instant
    }

    impl DoubleTapKSM {

        pub fn new(retap_threshold: Duration, hold_threshold: Duration) -> Self {
            Self {
                retap_threshold,
                hold_threshold,
                state: State::FirstTap,
                watched_key: KeyId::default(),
                key_conf: DoubleTapKeyConf::default(),
                creation: Instant::now(),
                release_timestamp: Instant::now(),
                initialized: false,
            }
        }
    }

    impl KeyStateMachine for DoubleTapKSM {
        type KeyConf = DoubleTapKeyConf;

        fn init_machine(&mut self, key_id: KeyId, key_conf: DoubleTapKeyConf) {
            self.watched_key = key_id;
            self.key_conf = key_conf;
            self.creation = Instant::now();
            self.initialized = true;
        }

        fn transition<'a>(&mut self, events: &'a [Event]) -> Option<KeyActionSet> {
            // first transition the current state to a new one
            match self.state {
                State::FirstTap => {
                    if events.contains(&Event::KeyRelease(self.watched_key)) {
                        self.release_timestamp = Instant::now();
                        self.state = State::FirstRelease;
                    }
                    else if (Instant::now() - self.creation) > self.hold_threshold {
                        self.state = State::Timeout;
                    }
                    else if events.iter().any(|event| event.is_key_press()) {
                        self.state = State::Timeout;
                    }
                },
                State::FirstRelease => {
                    if (Instant::now() - self.release_timestamp) > self.retap_threshold {
                        self.state = State::Timeout;
                    }
                    else if events.contains(&Event::KeyPress(self.watched_key)) {
                        self.state = State::Retap
                    }
                    else if events.iter().any(|event| event.is_key_press()) {
                        self.state = State::Timeout;
                    }
                },
                _ => () // NoOP because retap and timeout are accepting states
            }

            // return a value based on the new state
            match self.state {
                State::FirstTap => None,
                State::FirstRelease => None,
                State::Timeout => Some(self.key_conf.0),
                State::Retap => Some(self.key_conf.1),
            }
        }
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn test_mod() {
        }

    }
}


/*
   mod DoubleTapHoldStateM { }


   pub struct SMKeyboard {
   num_keys,
   output_actions_buff,
   active_keys,

   map_manager,

   statem: Option<Box<dyn KeyboardStateM>>,

   active_map: MapId,
   key_actions_buff,
   }

   impl SMKeyboard {
   pub fn new(initial_map: u8, action_buff_size: usize) -> Self { }
   }

   impl Keyboard for SMKeyboard {

   fn handle_state_machine<'a>(&mut self, events: &'a [Event]) {
   if let Some(machine) = self.state_machine {
   match machine.transition(events) {
   Some(actions) => {
   actions.into_iter().for_each(|action| self.key_actions_buff.push(action));
   self.state_machine = None;
   },
   None => {
   self.state_machine = machine;
   }
   }
   }
   }

// TODO should remove event associated to the key mapped to the state machine
// not doing so would put a lot of burden in the implementation of the state machine
// contracts which i don't want, i want to keep it simple
//
// should the state machines know which key it's looking for? yeah obviously
// the keyboard doesn't have to, it should query the state machine
//
// i need to handle the state machine, which will yield a key action set
// i need to handle the other events.
// each event should be mapped to a key action set
// at the end, handle all key action sets
// then produce the outputs

fn transition<'a>(&mut self, events: &mut 'a [Event]) -> &[Action] {
self.handle_state_machine(events);

// for remaining keys in events, get actionset / activation mode
// if key is not simple, create StateM for key.
// overwrite stateM if multiple stateful keys are sent
// otherwise, append action to output buffer
}

fn handle_event(event: Event) -> {
match event {
KeyPress(KeyId),
KeyRelease(KeyId)
}
}
}
*/
