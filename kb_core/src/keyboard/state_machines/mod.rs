/// Some key activation modes are stateful in nature and depends
/// on the state of the other keys in the keyboard to perform an action.
/// The KeyboardStateM is a trait that can be implemented for stateful
/// activation modes.
///
/// Each time a stateful key is pressed, a new state machine should be created
/// to handle that state.
/// Once the key has performed an action, the state machine will be in one of
/// its activation states and should be discarded.

mod hold_ksm;
mod double_tap_ksm;
mod double_tap_hold_ksm;

use crate::keyboard::{Event, Action};
use crate::keyboard::KeyId;
use crate::keys::{KeyConf, KeyActionSet};

pub use hold_ksm::{HoldKSM};
pub use double_tap_ksm::{DoubleTapKSM};
pub use double_tap_hold_ksm::{DoubleTapHoldKSM};


pub trait KeyStateMachine {

    /// Steps the state machine from the current events
    /// The state machine will either return a KeyActionSet or None.
    /// Once a state machine returns an KeyActionSet it has reached one 
    /// of its accepting states and should be discarded.
    fn transition<'a>(&mut self, event: Event) -> Option<KeyActionSet>;

    fn get_watched_key(&self) -> KeyId;
}

pub trait KSMInit: KeyStateMachine {
    type KeyConf;

    /// Initialize a State Machine instance.
    /// key_id indicates the key to which the state machine is attached.
    /// key_conf is the set of actions the key shall perform
    fn init_machine(&mut self, key_id: KeyId, key_conf: Self::KeyConf);
}
