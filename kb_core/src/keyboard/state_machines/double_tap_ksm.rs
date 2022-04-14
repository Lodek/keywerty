use std::time::{Instant, Duration};

use super::super::{Event};
use crate::keys::{KeyId, KeyActionSet, DoubleTapKeyConf};

use super::{KeyStateMachine, KSMInit};

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

    fn get_watched_key(&self) -> KeyId {
        self.watched_key
    }

    fn transition<'a>(&mut self, event: Event) -> Option<KeyActionSet> {
        // first transition the current state to a new one
        match self.state {
            State::FirstTap => {
                if event == Event::KeyRelease(self.watched_key) {
                    self.release_timestamp = Instant::now();
                    self.state = State::FirstRelease;
                }
                else if (Instant::now() - self.creation) > self.hold_threshold {
                    self.state = State::Timeout;
                }
                else if event.is_key_press() {
                    self.state = State::Timeout;
                }
            },
            State::FirstRelease => {
                if (Instant::now() - self.release_timestamp) > self.retap_threshold {
                    self.state = State::Timeout;
                }
                else if event == Event::KeyPress(self.watched_key) {
                    self.state = State::Retap
                }
                else if event.is_key_press() {
                    self.state = State::Timeout;
                }
            },
            _ => () // NoOP because retap and timeout are accepting states
        }

        // return a value based on the new state
        match self.state {
            State::FirstTap => None,
            State::FirstRelease => None,
            State::Timeout => Some(self.key_conf.tap),
            State::Retap => Some(self.key_conf.double_tap),
        }
    }
}

impl KSMInit for DoubleTapKSM {
    type KeyConf = DoubleTapKeyConf;

    fn init_machine(&mut self, key_id: KeyId, key_conf: DoubleTapKeyConf) {
        self.watched_key = key_id;
        self.key_conf = key_conf;
        self.creation = Instant::now();
        self.initialized = true;
    }
}

#[cfg(test)]
mod tests {
    // TODO write tests for Double Tap module
}
