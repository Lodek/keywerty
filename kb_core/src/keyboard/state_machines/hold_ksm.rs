/// Module for Key State Machine implementation for the `Hold` key configuration
use std::time::{Instant, Duration};

use super::super::{Event};
use crate::keyboard::KeyId;
use crate::keys::{KeyActionSet, HoldKeyConf};
use super::{KeyStateMachine, KSMInit};


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

    fn get_watched_key(&self) -> KeyId {
        self.watched_key
    }

    fn transition<'a>(&mut self, event: Event) -> Option<KeyActionSet> {

        if let State::Waiting = self.state {
            if (Instant::now() - self.creation) > self.release_delay {
                self.state = State::Hold;
            }

            else if event == Event::KeyRelease(self.watched_key) {
                self.state = State::Tap;
            }

            else if event.is_key_press() {
                self.state = State::Hold;
            }
        }

        match self.state {
            State::Waiting => None,
            State::Tap => Some(self.key_conf.tap),
            State::Hold => Some(self.key_conf.hold)
        }
    }
}

impl KSMInit for HoldKSM {
    type KeyConf = HoldKeyConf;

    fn init_machine(&mut self, key_id: KeyId, key_conf: HoldKeyConf) {
        self.watched_key = key_id;
        self.key_conf = key_conf;
        self.initialized = true;
        self.creation = Instant::now();
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
