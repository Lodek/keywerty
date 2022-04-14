/// Module defines types for keys with stateful activation modes

/// HID Scan codes
// probably use linux events instead
// TODO convert this to an enum cause pretty
pub type KeyCode = u8;
pub type KeyId = u8;
pub type LayerId = u8;

/// Activating a key triggers an action to occur.
/// An action can alter the internal state of the keyboard, or 
/// it may produce an output.
///
/// `AddKey`: indicates that the given keyboard Key should be sent to the host
/// `SetLayer`: sets the new active layer in the internal keyboard represtation
/// `NoOp`: does nothing
#[derive(PartialEq, Clone, Copy, Debug)]
// TODO huh, this still isn't necessary for the abstration. should I leave it here?
// it also seems to depend on the layer abstraction
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

impl KeyActionSet {
    fn get_actions(&self) -> Vec<KeyAction> {
        let mut actions = Vec::new();

        match self {
            KeyActionSet::Single(a1) => {
                actions.push(*a1);
            },
            KeyActionSet::Double(a1, a2) => {
                actions.push(*a1);
                actions.push(*a2);
            },
            KeyActionSet::Triple(a1, a2, a3) => {
                actions.push(*a1);
                actions.push(*a2);
                actions.push(*a3);
            },
        }
        actions
    }
}

impl Default for KeyActionSet {
    fn default() -> Self {
        Self::Single(KeyAction::default())
    }
}


#[derive(Debug, Clone, Copy)]
pub enum KeyConf {
    Tap(TapKeyConf),
    Hold(HoldKeyConf),
    DoubleTap(DoubleTapKeyConf),
    DoubleTapHold(DoubleTapHoldKeyConf),
}

impl Default for KeyConf {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TapKeyConf {
    pub tap: KeyActionSet,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HoldKeyConf {
    pub tap: KeyActionSet,
    pub hold: KeyActionSet,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoubleTapKeyConf {
    pub tap: KeyActionSet,
    pub double_tap: KeyActionSet,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoubleTapHoldKeyConf {
    pub tap: KeyActionSet,
    pub double_tap: KeyActionSet,
    pub hold: KeyActionSet,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DeadKeyConf {
    pub activation: KeyActionSet,
    pub retap: KeyActionSet,
}
