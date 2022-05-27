//! Module with definitions for Key configurations
pub use crate::mapper::LayerId;


/// A Key may have different different activation mechanisms.
/// KeyConf indicates a key's behavior once it's activated (ie a KeyPress event)
#[derive(Debug, Clone, Copy)]
pub enum KeyConf<T> {

    /// A Tap represents a key as most people are used to.
    /// Once it's pressed (key down) it performs an action.
    /// Upon being released it undo / stop performing the action.
    Tap(TapKeyConf<T>),

    /// A Hold key is a stateful key configuration
    /// where the Key performs different actions when either held or tapped.
    /// In this configuration, the Hold behavior is only fired
    /// after some other key is pressed or after a predetermined
    /// time interval passes by.
    Hold(HoldKeyConf<T>),

    /// An Eager Hold key is much like very similart to a Hold key, except
    /// the eager version will perform the `hold` action as 
    /// soon as the key is pressed.
    /// If the key is released before the hold activation timer,
    /// the performed action will be undone (through `KeyActionSet::invert`),
    /// and the tap action will be executed.
    EagerHold(HoldKeyConf<T>),

    /// A Double Tap is a composed key configuration where the key performs
    /// one action if pressed/released and another if pressed, released and quickly retapped in sucession.
    DoubleTap(DoubleTapKeyConf<T>),

    /// Double Tap Hold merges the behavior of Hold and Double Tap.
    /// The `hold` action is sent if the key is pressed and held for a specified threshold.
    /// Upon key release, if the key is retapped before the retap threshold, it will perform the
    /// `double_tap` action, otherwise it will perform the `tap` action.
    ///
    /// This key configuration is often used to map the Caps Lock key into Ctrl for `hold`,
    /// ESC for `tap` and Caps Lock for `double_tap`
    DoubleTapHold(DoubleTapHoldKeyConf<T>),
}


/// KeyAction models the different side effects a Key can have when activated.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum KeyAction<T> {
    /// Indicates that the Keyboard should send some data for `T`.
    /// Should be equivalent to an `Action::SendKey`.
    SendKey(T),

    /// Indicate that the Keyboard should stop sending `T`.
    /// Translates to an `Action::Stop`.
    StopKey(T),

    /// Push the layer given by `LayerId` onto the LayerStack.
    PushLayer(LayerId),

    /// Remove the first occurence of `LayerId` from the layer stack.
    PopLayer(LayerId),

    /// No operation action
    NoOp,
}

impl<T> Into<KeyActionSet<T>> for KeyAction<T> {
    fn into(self) -> KeyActionSet<T> {
        KeyActionSet::Single(self)
    }
}

impl<T: Clone> KeyAction<T> {

    /// Convenience method to map out the inverse of a KeyAction.
    /// Conceptually the inverse of an action undoes or cancels
    /// what the original action did.
    pub fn invert(&self) -> Self {
        // TODO not sure if this still makes sense.
        // It's convenient but may cause confusion.
        match self {
            Self::SendKey(data) => Self::StopKey(data.clone()),
            Self::StopKey(data) => Self::SendKey(data.clone()),
            Self::PushLayer(layer_id) => Self::PopLayer(*layer_id),
            Self::PopLayer(layer_id) => Self::PushLayer(*layer_id),
            Self::NoOp => Self::NoOp,
        }
    }
}

impl<T> Default for KeyAction<T> {

    /// KeyAction defaults to NoOp
    fn default() -> Self {
        KeyAction::NoOp
    }
}


/// A group of KeyActions that will be triggered once a key is activated
/// It's often useful / interesting for a Key to perform more than
/// one action at a time.
/// KeyActionSet encapsulates this scenario.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum KeyActionSet<T> {
    // TODO this kinda doesn't make a whole lot of sense.
    // It does but it doesn't. Should revisit this at some point.

    // TODO Understand how enum variants are stored in memory.
    // Does it allocate memory for the biggest variant?
    Single(KeyAction<T>),
    Double(KeyAction<T>, KeyAction<T>),
    Triple(KeyAction<T>, KeyAction<T>, KeyAction<T>),
}

impl<T: Clone> KeyActionSet<T> {
    
    /// Collect actions in the action set and return a Vector of `KeyAction`s.
    pub fn get_actions(&self) -> Vec<KeyAction<T>> {
        let mut actions = Vec::new();

        match self {
            KeyActionSet::Single(a1) => {
                actions.push(a1.clone());
            },
            KeyActionSet::Double(a1, a2) => {
                actions.push(a1.clone());
                actions.push(a2.clone());
            },
            KeyActionSet::Triple(a1, a2, a3) => {
                actions.push(a1.clone());
                actions.push(a2.clone());
                actions.push(a3.clone());
            },
        }
        actions
    }

    /// Return action set with every KeyAction inverted.
    pub fn invert(&self) -> KeyActionSet<T> {
        match self {
            KeyActionSet::Single(a1) => KeyActionSet::Single(a1.invert()),
            KeyActionSet::Double(a1, a2) =>  KeyActionSet::Double(a1.invert(), a2.invert()),
            KeyActionSet::Triple(a1, a2, a3) => KeyActionSet::Triple(a1.invert(), a2.invert(), a3.invert()),
        }
    }
}

impl<T> Default for KeyActionSet<T> {

    /// KeyActionSet defaults to a Single NoOp action
    fn default() -> Self {
        Self::Single(KeyAction::default())
    }
}




/// Configuration for a Tap keyconf, tap keys have a single action.
#[derive(Clone, Copy, Debug)]
pub struct TapKeyConf<T> {
    pub tap: KeyActionSet<T>,
}

impl<T> Default for TapKeyConf<T> {
    fn default() -> Self {
        Self {
            tap: KeyActionSet::default()
        }
    }
}


/// Actions for a hold or eager hold key conf.
/// These configurations perform two actions, one for tap and another for hold.
#[derive(Clone, Copy, Debug)]
pub struct HoldKeyConf<T> {
    pub tap: KeyActionSet<T>,
    pub hold: KeyActionSet<T>,
}

impl<T> Default for HoldKeyConf<T> {
    fn default() -> Self {
        Self {
            tap: KeyActionSet::default(),
            hold: KeyActionSet::default()
        }
    }
}


/// Actions for a Double tap key configuration.
/// One action for a key press another for a tap, release and retap cycle.
#[derive(Clone, Copy, Debug)]
pub struct DoubleTapKeyConf<T> {
    pub tap: KeyActionSet<T>,
    pub double_tap: KeyActionSet<T>,
}

impl<T> Default for DoubleTapKeyConf<T> {
    fn default() -> Self {
        Self {
            tap: KeyActionSet::default(),
            double_tap: KeyActionSet::default()
        }
    }
}


/// Actions for a double-tap-hold configuration.
/// one action for a tap, one for a hold and another for a double tap activation.
#[derive(Clone, Copy, Debug)]
pub struct DoubleTapHoldKeyConf<T> {
    pub tap: KeyActionSet<T>,
    pub double_tap: KeyActionSet<T>,
    pub hold: KeyActionSet<T>,
}

impl<T> Default for DoubleTapHoldKeyConf<T> {
    fn default() -> Self {
        Self {
            tap: KeyActionSet::default(),
            double_tap: KeyActionSet::default(),
            hold: KeyActionSet::default(),
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct DeadKeyConf<T> {
    pub activation: KeyActionSet<T>,
    pub retap: KeyActionSet<T>,
}

impl<T> Default for DeadKeyConf<T> {
    fn default() -> Self {
        Self {
            activation: KeyActionSet::default(),
            retap: KeyActionSet::default()
        }
    }
}
