/// Module for Key State Machine implementation for the `Hold` key configuration
use std::time::{Duration, Instant};

use super::KeyStateMachine;
use crate::keyboard::smkb::helpers;
use crate::keyboard::Event;
use crate::keys::HoldKeyConf;
use crate::keys::KeyActionSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Created,
    Waiting,
    Hold,
    Released,
    Finished,
}

#[derive(Debug)]
pub struct EagerHoldKSM<KeyId, T> {
    watched_key: KeyId,
    state: State,
    key_conf: HoldKeyConf<T>,
    timer_start: Instant,
    release_delay: Duration,
    cleanup_actions: [KeyActionSet<T>; 1],
}

impl<KeyId, T> EagerHoldKSM<KeyId, T> {
    pub fn new(release_delay: Duration, watched_key: KeyId, conf: HoldKeyConf<T>) -> Self {
        return Self {
            release_delay,
            watched_key,
            timer_start: Instant::now(),
            state: State::Created,
            key_conf: conf,
            cleanup_actions: [KeyActionSet::default()],
        };
    }
}

impl<KeyId, T> KeyStateMachine<KeyId, T> for EagerHoldKSM<KeyId, T>
where
    KeyId: PartialEq,
    T: Clone,
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

        match self.state {
            State::Created => {
                if helpers::is_watched_key_pressed(self, event) {
                    // send hold action
                    self.timer_start = Instant::now();
                    self.state = State::Waiting;
                    let action = &self.key_conf.hold;
                    self.cleanup_actions[0] = action.invert();
                    Some(action.clone())
                } else {
                    None
                }
            }
            State::Waiting => {
                // held till timeout or other key was pressed
                // noop
                if (Instant::now() - self.timer_start) >= self.release_delay
                    || matches!(event, Event::KeyPress(key_id) if key_id != watched_key)
                {
                    self.state = State::Hold;
                    None
                }
                // key released before timer means tap
                // undo the held key
                else if matches!(event, Event::KeyRelease(key_id) if key_id == watched_key) {
                    self.state = State::Released;
                    Some(self.key_conf.hold.invert())
                } else {
                    None
                }
            }
            State::Released => {
                // when released, send the tap action
                self.state = State::Finished;
                self.cleanup_actions[0] = self.key_conf.tap.invert();
                Some(self.key_conf.tap.clone())
            }
            State::Hold => {
                // if key was held, wait until its released
                if matches!(event, Event::KeyRelease(key_id) if key_id == watched_key) {
                    self.state = State::Finished;
                }
                None
            }
            State::Finished => None,
        }
    }

    fn get_cleanup_actions(&self) -> &[KeyActionSet<T>] {
        &self.cleanup_actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::KeyAction;
    use std::thread::sleep;
    use std::time::Duration;

    const watched_key: u8 = 1;
    const tap_key_code: u8 = 10;
    const hold_key_code: u8 = 20;

    fn build_ksm() -> EagerHoldKSM<u8, u8> {
        let timeout = Duration::from_millis(2);
        let tap_action = KeyActionSet::Single(KeyAction::SendKey(tap_key_code));
        let hold_action = KeyActionSet::Single(KeyAction::SendKey(hold_key_code));
        let conf = HoldKeyConf {
            tap: tap_action,
            hold: hold_action,
        };
        let mut machine = EagerHoldKSM::new(timeout, watched_key, conf);
        machine
    }

    #[test]
    fn test_key_press_eagerly_sends_hold_action_and_after_timeout_undoes_hold_action() {
        let mut machine = build_ksm();

        // When I transition machine by sending key press event
        let opt = machine.transition(&Event::KeyPress(watched_key));
        assert_eq!(
            opt.unwrap(),
            KeyActionSet::Single(KeyAction::SendKey(hold_key_code))
        );
        assert!(!machine.is_finished());

        // When I poll before timeout
        for i in [0..2] {
            sleep(Duration::from_nanos(500));
            let opt = machine.transition(&Event::Poll);
            assert!(opt.is_none());
            assert!(!machine.is_finished());
        }

        // When I poll after timeout
        sleep(Duration::from_millis(2));
        let opt = machine.transition(&Event::Poll);
        assert!(opt.is_none());
        assert!(!machine.is_finished());

        // when machine key is released
        let opt = machine.transition(&Event::KeyRelease(watched_key));
        assert!(opt.is_none());
        assert!(machine.is_finished());
        // cleanup action is the inverse of hold action
        let cleanup = machine.get_cleanup_actions();
        assert_eq!(cleanup.len(), 1);
        assert_eq!(
            cleanup[0],
            KeyActionSet::Single(KeyAction::StopKey(hold_key_code))
        );
    }

    #[test]
    fn test_key_press_then_release_undoes_hold_action_and_sends_tap() {
        let mut machine = build_ksm();

        // When I transition machine by sending key press event
        let opt = machine.transition(&Event::KeyPress(watched_key));
        assert_eq!(
            opt.unwrap(),
            KeyActionSet::Single(KeyAction::SendKey(hold_key_code))
        );
        assert!(!machine.is_finished());

        // When I release key then it undoes hold action
        let opt = machine.transition(&Event::KeyRelease(watched_key));
        assert_eq!(
            opt.unwrap(),
            KeyActionSet::Single(KeyAction::StopKey(hold_key_code))
        );
        assert!(!machine.is_finished());

        // when i poll then it sends tap action and machine is finished
        let opt = machine.transition(&Event::Poll);
        assert_eq!(
            opt.unwrap(),
            KeyActionSet::Single(KeyAction::SendKey(tap_key_code))
        );
        assert!(machine.is_finished());

        // cleanup undoes tap
        let cleanup = machine.get_cleanup_actions();
        assert_eq!(cleanup.len(), 1);
        assert_eq!(
            cleanup[0],
            KeyActionSet::Single(KeyAction::StopKey(tap_key_code))
        );
    }
}
