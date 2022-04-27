/// Dead key behavior implementation
use keyboard_interface::{Event<Id>};
use crate::keyboard::KeyId;
use keyboard_interface::map::{KeyActionSet, DeadKeyConf};
use crate::statem::{KeyStateMachine, KSMInit};

enum State {

}

/// Key state machine that implements a dead key behavior.
/// Dead keys are keys that upon trigger:
/// 1. apply a key action set once the key (call it Key `d`) is released
/// 2. await for any other key press (call it Key `k`) and apply its actionset
/// 3. undo `d`'s key actionset after `k` is released
pub struct DeadKeyKMS {

}

impl KeyStateMachine for DeadKeyKSM {

    fn transition<'a>(&mut self, event: Event<Id>) -> Option<KeyActionSet> {
        None
    }

    fn get_watched_key(&self) -> KeyId {
        0
    }
}

impl KSMInit for DeadKeyKSM {
    type KeyConf = DeadKeyConf;

    fn init_machine(&mut self, key_id: KeyId, key_conf: DeadKeyConf) {

    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {

    }
}
