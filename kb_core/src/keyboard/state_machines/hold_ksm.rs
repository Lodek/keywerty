/// Module for Key State Machine implementation for the `Hold` key configuration
use std::time::{Instant, Duration};

use crate::keys::KeyActionSet;
use crate::keys::HoldKeyConf;
use crate::keyboard::Event;
use super::KeyStateMachine;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Created,
    Waiting,
    Hold,
    Released,
    Finished,
}

#[derive(Debug)]
pub struct HoldKSM<KeyId, T> {
    watched_key: KeyId,
    state: State,
    key_conf: HoldKeyConf<T>,
    timer_start: Instant,
    release_delay: Duration,
}

impl<KeyId, T> HoldKSM<KeyId, T> {
    pub fn new(release_delay: Duration, watched_key: KeyId, conf: HoldKeyConf<T>) -> Self {
        return Self {
            release_delay,
            watched_key,
            timer_start: Instant::now(),
            state: State::Created,
            key_conf: conf,
        }
    }
}

impl<KeyId, T> KeyStateMachine<KeyId, T> for HoldKSM<KeyId, T> 
where KeyId: PartialEq,
      T: Clone
{

    fn get_watched_key(&self) -> &KeyId {
        &self.watched_key
    }
    
    fn is_finished(&self) -> bool {
        matches!(self.state, State::Finished)
    }

    fn transition(&mut self, event: &Event<KeyId>) -> Option<KeyActionSet<T>> {
        if self.is_finished() {
            return None;
        }

        let watched_key = self.get_watched_key();

        // TODO define macros / functions to make conditions
        // more legible
        match self.state {
            State::Created => {
                if matches!(event, Event::KeyPress(key_id) if key_id == watched_key) {
                    self.timer_start = Instant::now();
                    self.state = State::Waiting;
                }
                None
            },
            State::Waiting => {
                // pressed till timeout or other key was pressed
                // hold
                if (Instant::now() - self.timer_start) >= self.release_delay || 
                    matches!(event, Event::KeyPress(key_id) if key_id != watched_key)
                {
                    self.state = State::Hold;
                    Some(self.key_conf.hold.clone())
                }
                // key released before timer means tap
                else if matches!(event, Event::KeyRelease(key_id) if key_id == watched_key) {
                    self.state = State::Released;
                    Some(self.key_conf.tap.clone())
                }
                else {
                    None
                }
            },
            State::Released => {
                // after released, go to finished
                self.state = State::Finished;
                None
            },
            State::Hold => {
                // if key was held, wait until its released
                if matches!(event, Event::KeyRelease(key_id) if key_id == watched_key) {
                    self.state = State::Finished;
                }
                None
            },
            State::Finished => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::thread::sleep;
    use crate::keys::KeyAction;

    const watched_key: u8 = 1;
    const tap_key_code: u8 = 10;
    const hold_key_code: u8 = 20;

    fn build_ksm() -> HoldKSM<u8, u8> {
        let timeout = Duration::from_millis(2);
        let tap_action = KeyActionSet::Single(KeyAction::SendKey(tap_key_code));
        let hold_action = KeyActionSet::Single(KeyAction::SendKey(hold_key_code));
        let conf = HoldKeyConf { tap: tap_action, hold: hold_action };
        let mut machine = HoldKSM::new(timeout, watched_key, conf);
        machine
    }

    #[test]
    fn test_key_timeout_with_hold_kms() {
        let mut machine = build_ksm();

        // When I transition machine by sending key press event
        let opt = machine.transition(&Event::KeyPress(watched_key));
        assert!(opt.is_none());
        assert!(!machine.is_finished());

        // When I sleep for timeout
        // And machine is polled 
        for i in [0..2] {
            sleep(Duration::from_nanos(500));
            let opt = machine.transition(&Event::Poll);
            assert!(opt.is_none());
            assert!(!machine.is_finished());
        }

        // when i poll after timeout
        sleep(Duration::from_millis(2));
        let opt = machine.transition(&Event::Poll);
        assert_eq!(opt.unwrap(), KeyActionSet::Single(KeyAction::SendKey(hold_key_code)));
        assert!(!machine.is_finished());

        // when machine is polled 
        let opt = machine.transition(&Event::Poll);
        assert!(opt.is_none());
        assert!(!machine.is_finished());

        // when machine key is released
        let opt = machine.transition(&Event::KeyRelease(watched_key));
        assert!(opt.is_none());
        assert!(machine.is_finished());
    }

    #[test]
    fn test_pressing_other_key_with_hold_kms_means_key_was_held() {
        let mut machine = build_ksm();

        // When I start machine by sending key press event
        let opt = machine.transition(&Event::KeyPress(watched_key));
        assert!(opt.is_none());
        assert!(!machine.is_finished());

        // When another key is pressed
        let opt = machine.transition(&Event::KeyPress(255));
        assert_eq!(opt.unwrap(), KeyActionSet::Single(KeyAction::SendKey(hold_key_code)));
        assert!(!machine.is_finished());

        // when machine is polled 
        let opt = machine.transition(&Event::Poll);
        assert!(opt.is_none());
        assert!(!machine.is_finished());

        // when machine key is released
        let opt = machine.transition(&Event::KeyRelease(watched_key));
        assert!(opt.is_none());
        assert!(machine.is_finished());
    }

    #[test]
    fn test_releasing_watched_key_before_timeout_sends_tap() {
        let mut machine = build_ksm();

        // When I start machine by sending key press event
        let opt = machine.transition(&Event::KeyPress(watched_key));
        assert!(opt.is_none());
        assert!(!machine.is_finished());

        // When I release the watched key
        let opt = machine.transition(&Event::KeyRelease(watched_key));
        assert_eq!(opt.unwrap(), KeyActionSet::Single(KeyAction::SendKey(tap_key_code)));
        assert!(!machine.is_finished());

        // when machine is polled 
        let opt = machine.transition(&Event::Poll);
        assert!(opt.is_none());
        assert!(machine.is_finished());
    }

}
