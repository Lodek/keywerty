use std::fmt::Debug;

use super::KeyStateMachine;
use crate::keyboard::Event;
use crate::keys::TapKeyConf;
use crate::keys::KeyActionSet;


#[derive(Debug)]
pub struct TapKSM<KeyId, T> {
    finished: bool,
    watched_key: KeyId,
    conf: TapKeyConf<T>,
    cleanup_actions: [KeyActionSet<T>; 1]
}

impl<KeyId, T> TapKSM<KeyId, T> 
where T: Clone
{
    pub fn new(watched_key: KeyId, key_conf: TapKeyConf<T>) -> Self {
        Self {
            cleanup_actions: [(&key_conf).tap.invert()],
            conf: key_conf,
            finished: false,
            watched_key,
        }
    }
}

impl<KeyId, T> KeyStateMachine<KeyId, T> for TapKSM<KeyId, T> 
where KeyId: PartialEq + Debug,
      T: Clone
{

    fn transition<'a>(&mut self, event: &Event<KeyId>) -> Option<KeyActionSet<T>> {
        if self.is_finished() {
            return None;
        }

        let watched_key = self.get_watched_key();

        match event {
            Event::KeyPress(key_id) if key_id == watched_key => {
                Some(self.conf.tap.clone())
            },
            Event::KeyRelease(key_id) if key_id == watched_key => {
                self.finished = true;
                None
            },
            _ => None,
        }
    }

    fn get_watched_key(&self) -> &KeyId {
        &self.watched_key
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn get_cleanup_actions(&self) -> &[KeyActionSet<T>] {
        &self.cleanup_actions
    }
}
