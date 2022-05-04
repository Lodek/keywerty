use std::fmt::Debug;

use crate::keyboard::state_machines::KeyStateMachine;
use crate::keyboard::state_machines::KSMInit;
use crate::keyboard::state_machines::KSMHelper;
use crate::keyboard::Event;
use crate::keys::TapKeyConf;
use crate::keys::KeyActionSet;


#[derive(Debug)]
pub struct TapKSM<KeyId, T> {
    initialized: bool,
    accepting: bool,
    watched_key: Option<KeyId>,
    conf: TapKeyConf<T>
}

impl<KeyId, T> TapKSM<KeyId, T> {
    pub fn new() -> Self {
        Self {
            initialized: false,
            accepting: false,
            watched_key: None,
            conf: TapKeyConf::default()
        }
    }
}

impl<KeyId, T> KSMInit<KeyId> for TapKSM<KeyId, T>
{
    type KeyConf = TapKeyConf<T>;

    fn init_machine(&mut self, key_id: KeyId, key_conf: Self::KeyConf) {
        self.initialized = true;
        self.conf = key_conf;
        self.watched_key = Some(key_id);
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl<KeyId, T> KeyStateMachine<KeyId, T> for TapKSM<KeyId, T> 
where KeyId: PartialEq + Debug,
      T: Clone
{

    fn transition<'a>(&mut self, event: &Event<KeyId>) -> Option<KeyActionSet<T>> {
        if !self.can_transition() {
            None
        }
        else if matches!(event, Event::KeyPress(key_id) if key_id == self.get_watched_key().unwrap()) {
            // TODO debug log
            eprintln!("tap event for event: {:?}", event);
            self.accepting = true;
            Some(self.conf.tap.clone())
        }
        else {
            None
        }
    }

    fn get_watched_key(&self) -> Option<&KeyId> {
        self.watched_key.as_ref()
    }

    fn is_finished(&self) -> bool {
        self.accepting
    }
}
