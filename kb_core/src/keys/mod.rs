/// Module defines types for keys with stateful activation modes


#[derive(Debug)]
pub enum KeyConf {
    Tap(TapKeyConf),
    Hold(HoldKeyConf),
    DoubleTap(DoubleTapKeyConf),
    DoubleTapHold(DoubleTapHoldKeyConf),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TapKeyConf<T> {
    pub tap: <T>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HoldKeyConf<T> {
    pub tap: T,
    pub hold: T,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoubleTapKeyConf<T> {
    pub tap: T,
    pub double_tap: T,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoubleTapHoldKeyConf<T> {
    pub tap: T,
    pub double_tap: T,
    pub hold: T,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DeadKeyConf<T> {
    pub activation: T,
    pub retap: T,
}

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

enum KeyAction<M, A> {
    Mutation(M),
    Action(A),
    NoOp
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

pub trait KeyActionSet {
    fn get_actions(&self) -> &[KeyAction];
}

