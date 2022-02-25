/// Keyboard module with main abstractions that rule a keyboard

/// HID Scan codes
// TODO convert this to an enum cause pretty
pub type KeyCode = u8;

pub type KeyId = u8;

/// Set of events that a keyboard respond to. (inputs)
#[derive(PartialEq, Debug, Clone)]
pub enum Event {
    KeyPress(KeyId),
    KeyRelease(KeyId)
}

impl Event {
    pub fn is_key_press(&self) -> bool {
        match self {
            Event::KeyPress(_) => true,
            _ => false
        }
    }
}

/// Set of actions a keyboard perform as consequence of inputs. (outputs)
#[derive(Debug, Clone)]
pub enum Action {
    SendCode(KeyCode),
    Stop(KeyCode)
}


/// Abstraction for a physical keyboard.
/// Conceptually a keyboard contains keys, each identified with an id.
///
/// The keyboard receives `Event`s as input and returns a set
/// of `Action`s indicating what should be done.
///
/// It can be thought of as a state machine, each time it receives an input
/// it goes to a different state and produces an output
pub trait Keyboard {
    fn transition<'a>(&mut self, events: &'a [Event]) -> &[Action];
}


pub mod map {
    use super::keyboard::{KeyCode, KeyId};

    pub type LayerId = u8;

    /// Activating a key triggers an action to occur.
    /// An action can have alter the internal state of the keyboard, or 
    /// it may produce an output.
    ///
    /// `AddKey`: indicates that the given keyboard Key should be sent to the host
    /// `SetLayer`: sets the new active layer in the internal keyboard represtation
    /// `NoOp`: does nothing
    #[derive(PartialEq, Clone, Copy, Debug)]
    pub enum KeyAction {
        AddKey(KeyCode),
        SetLayer(LayerId),
        NoOp,

        // Some actions were mapped as being useful, however they are a bit
        // of an edge case. As such, they won't be implemented in this iteration.
        //
        // ToggleKey,
        // ToggleLayer,
        // RemoveKey, // removes key from set of active keys
    }

    impl Default for KeyAction {
        fn default() -> Self {
            KeyAction::NoOp
        }
    }

    /// A group of KeyActions that will be triggered once a key is activated
    #[derive(PartialEq, Clone, Copy, Debug)]
    pub enum KeyActionSet {
        // TODO Understand how enum variants are stored in memory
        Single(KeyAction),
        Double(KeyAction, KeyAction),
        Triple(KeyAction, KeyAction, KeyAction),
    }

    impl Default for KeyActionSet {
        fn default() -> Self {
            Self::Single(KeyAction::default())
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct TapKeyConf (pub KeyActionSet);

    #[derive(Clone, Copy, Debug, Default)]
    pub struct HoldKeyConf (pub KeyActionSet, pub KeyActionSet);

    #[derive(Clone, Copy, Debug, Default)]
    pub struct DoubleTapKeyConf(pub KeyActionSet, pub KeyActionSet);

    #[derive(Clone, Copy, Debug, Default)]
    pub struct DoubleTapHoldKeyConf(pub KeyActionSet, pub KeyActionSet, pub KeyActionSet);


    // TODO I need to figure out if this is efficient, but this seems alright as far as structure goes.
    #[derive(Debug)]
    pub enum KeyConf {
        Tap(TapKeyConf),
        Hold(HoldKeyConf),
        DoubleTap(DoubleTapKeyConf),
        DoubleTapHold(DoubleTapHoldKeyConf),
    }

    pub trait LayerMap {
        fn get(layer: LayerId, key: KeyId) -> KeyConf;
    }
}
