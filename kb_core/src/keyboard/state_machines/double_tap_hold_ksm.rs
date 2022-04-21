use std::time::{Instant, Duration};

use super::super::Event;
use crate::keys::{KeyActionSet, DoubleTapHoldKeyConf};
use crate::keyboard::KeyId;

use super::{KeyStateMachine, KSMInit};

enum State {
    Waiting,
    Released,
    Hold,
    DoubleTap,
    Tap
}

pub struct DoubleTapHoldKSM<T> {
    state: State,
    key_conf: DoubleTapHoldKeyConf<T>,
    watched_key: KeyId,
    hold_threshold: Duration,
    retap_threshold: Duration,
    created: Instant,
    released: Instant,
}

impl<T: Copy> DoubleTapHoldKSM<T> {
    pub fn new(hold_threshold: Duration, retap_threshold: Duration) -> Self {
        Self {
            hold_threshold,
            retap_threshold,
            state: State::Waiting,
            key_conf: DoubleTapHoldKeyConf::default(),
            watched_key: KeyId::default(),
            created: Instant::now(),
            released: Instant::now()
        }
    }
}

impl<T: Copy> KeyStateMachine<T> for DoubleTapHoldKSM<T> {

    fn get_watched_key(&self) -> KeyId {
        self.watched_key
    }

    fn transition<'a>(&mut self, event: Event) -> Option<KeyActionSet<T>> {
        match self.state {
            //TODO figure out how to humanize these checks (macro or inline function?)
            State::Waiting => {
                // check hold expiration -> send to hold
                // check other key tap -> send to hold
                // check watched_key release -> send to released
            },
            State::Released => {
                // check retap_threshold -> send to tap
                // check other key press -> send to tap
                // check key retap -> send to double tap
            }
            _ => (),
        }

        match self.state {
            State::Waiting => None,
            State::Released => None,
            State::Tap => Some(self.key_conf.tap),
            State::Hold => Some(self.key_conf.hold),
            State::DoubleTap => Some(self.key_conf.double_tap),
        }
    }

}

impl<T: Copy> KSMInit<T> for DoubleTapHoldKSM<T> {
    type KeyConf = DoubleTapHoldKeyConf<T>;

    fn init_machine(&mut self, key_id: KeyId, key_conf: DoubleTapHoldKeyConf<T>) {
        self.watched_key = key_id;
        self.key_conf = key_conf;
        self.created = Instant::now();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
    }
}
