use super::{KeyCode, KeyId};

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

#[derive(Clone, Copy, Debug, Default)]
pub struct DeadKeyConf(pub KeyActionSet, pub KeyActionSet);


// TODO I need to figure out if this is efficient, but this seems alright as far as structure goes.
#[derive(Debug)]
pub enum KeyConf {
    Tap(TapKeyConf),
    Hold(HoldKeyConf),
    DoubleTap(DoubleTapKeyConf),
    DoubleTapHold(DoubleTapHoldKeyConf),
}

pub trait LayerMapper {
    fn get_conf(&self, layer: LayerId, key: KeyId) -> KeyConf;
}


/// Simple Mapper implementation to aid testing.
/// Mapper returns `num_keys * layer` + `key`, which yields
/// a deterministic and unique keycode for combination.
/// (So long as the result is not grater than 2^8)
pub struct SimpleMapper {
    num_keys: u8
}

impl SimpleMapper {
    pub fn new(num_keys: u8) -> Self {
        SimpleMapper {num_keys}
    }
}

impl LayerMapper for SimpleMapper {
    fn get_conf(&self, layer: LayerId, key: KeyId) -> KeyConf {
        let key_code = layer * self.num_keys + key;
        let key_action = KeyAction::AddKey(key_code);
        KeyConf::Tap(TapKeyConf(KeyActionSet::Single(key_action)))
    }
}
