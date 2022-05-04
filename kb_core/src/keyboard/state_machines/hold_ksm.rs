/// Module for Key State Machine implementation for the `Hold` key configuration
use std::time::{Instant, Duration};

use crate::keys::KeyActionSet;
use crate::keys::HoldKeyConf;
use crate::keyboard::Event;
use super::KeyStateMachine;
use super::KSMHelper;
use super::KSMInit;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Waiting,
    Hold,
    Tap
}

#[derive(Debug)]
pub struct HoldKSM<KeyId, T> {
    watched_key: Option<KeyId>,
    state: State,
    key_conf: HoldKeyConf<T>,
    creation: Instant,
    release_delay: Duration,
    initialized: bool,
}

impl<KeyId, T> HoldKSM<KeyId, T> {
    pub fn new(release_delay: Duration) -> Self {
        return Self {
            creation: Instant::now(),
            release_delay,
            state: State::Waiting,
            key_conf: HoldKeyConf::default(),
            watched_key: None,
            initialized: false,
        }
    }
}

impl<KeyId, T> KeyStateMachine<KeyId, T> for HoldKSM<KeyId, T> 
where KeyId: PartialEq,
      T: Clone
{

    fn get_watched_key(&self) -> Option<&KeyId> {
        self.watched_key.as_ref()
    }
    
    fn is_finished(&self) -> bool {
        !matches!(self.state, State::Waiting)
    }

    fn transition(&mut self, event: &Event<KeyId>) -> Option<KeyActionSet<T>> {
        if !self.can_transition() {
            return None;
        }

        let watched_key = self.get_watched_key().unwrap();

        if let State::Waiting = self.state {
            if (Instant::now() - self.creation) > self.release_delay {
                self.state = State::Hold;
            }
            else if matches!(event, Event::KeyRelease(key_id) if key_id == watched_key) {
                self.state = State::Tap;
            }
            else if matches!(event, Event::KeyPress(_)) {
                self.state = State::Hold;
            }
        }

        match self.state {
            State::Waiting => None,
            State::Tap => Some(self.key_conf.tap.clone()),
            State::Hold => Some(self.key_conf.hold.clone())
        }
    }
}

impl<KeyId, T> KSMInit<KeyId> for HoldKSM<KeyId, T> 
{
    type KeyConf = HoldKeyConf<T>;

    fn init_machine(&mut self, key_id: KeyId, key_conf: HoldKeyConf<T>) {
        self.watched_key = Some(key_id);
        self.key_conf = key_conf;
        self.initialized = true;
        self.creation = Instant::now();
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/*
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
*/
