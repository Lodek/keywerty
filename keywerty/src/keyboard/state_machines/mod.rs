/// Some key activation modes are stateful in nature and depends
/// on the state of the other keys in the keyboard to perform an action.
/// The KeyboardStateM is a trait that can be implemented for stateful
/// activation modes.
///
/// Each time a stateful key is pressed, a new state machine should be created
/// to handle that state.
/// Once the key has performed an action, the state machine will be in one of
/// its activation states and should be discarded.

mod tap_ksm;
mod hold_ksm;
//mod double_tap_ksm;
//mod double_tap_hold_ksm;

use crate::keyboard::{Event, Action};
use crate::keys::{KeyConf, KeyActionSet};

pub use tap_ksm::TapKSM;
pub use hold_ksm::{HoldKSM};
//pub use double_tap_ksm::{DoubleTapKSM};
//pub use double_tap_hold_ksm::{DoubleTapHoldKSM};


/// KeyStateMachine (KSM) abstracts a key's internal activation mechanism.
///
/// Conceptually a KSM will respond to events and may generate
/// `KeyAction`s in response to them.
/// Each KSM handles the activation of *one* key, the "watched key".
///
/// Upon implementation specific conditions, a KSM will reach its
/// final state which means should not return any new events.
///
/// Semantically, a KSM *may* respond to `Event`s with `KeyAction`,
/// it *should* have a finished state and once this state is finished
/// the machine *should* be considered disposable.
///
/// A machine *should* be initialized before being used
/// and attempting to transition an uninitialized machine
/// *must not* return any actions.
pub trait KeyStateMachine<KeyId, T> {

    /// Steps the state machine from the current events
    /// Each step may return a KeyActionSet.
    ///
    /// An unitialized machine *should not* return any events.
    fn transition<'a>(&mut self, event: &Event<KeyId>) -> Option<KeyActionSet<T>>;

    /// Return the key for which the KSM is reponsible.
    fn get_watched_key(&self) -> &KeyId;

    /// Check whether the machine's current state is one of its accepting states.
    /// A state machine in an accepting state is finished and can be discarded
    fn is_finished(&self) -> bool;
}
