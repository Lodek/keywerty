/// Module defines types for keys with stateful activation modes

pub type LayerId = u8;

/// Activating a key triggers an action to occur.
/// An action can alter the internal state of the keyboard, or 
/// it may produce an output.
///
/// `AddKey`: indicates that the given keyboard Key should be sent to the host
/// `SetLayer`: sets the new active layer in the internal keyboard represtation
/// `NoOp`: does nothing
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum KeyAction<T> {
    SendKey(T),
    StopKey(T),
    PushLayer(LayerId),
    PopLayer(LayerId),
    NoOp,
    ToggleKey(T),
    ToggleLayer(LayerId),
}

impl<T: Copy> KeyAction<T> {
    pub fn invert(&self) -> Self {
        match self {
            Self::SendKey(key_id) => Self::StopKey(*key_id),
            Self::StopKey(key_id) => Self::SendKey(*key_id),
            Self::PushLayer(layer_id) => Self::PopLayer(*layer_id),
            Self::PopLayer(layer_id) => Self::PushLayer(*layer_id),
            Self::NoOp => Self::NoOp,
            Self::ToggleKey(key_id) => Self::ToggleKey(*key_id),
            Self::ToggleLayer(layer_id) => Self::ToggleLayer(*layer_id),
        }
    }
}

impl<T> Default for KeyAction<T> {
    fn default() -> Self {
        KeyAction::NoOp
    }
}


/// A group of KeyActions that will be triggered once a key is activated
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum KeyActionSet<T> {
    // TODO Understand how enum variants are stored in memory
    Single(KeyAction<T>),
    Double(KeyAction<T>, KeyAction<T>),
    Triple(KeyAction<T>, KeyAction<T>, KeyAction<T>),
}

impl<T: Copy> KeyActionSet<T> {
    pub fn get_actions(&self) -> Vec<KeyAction<T>> {
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

    pub fn invert(&self) -> KeyActionSet<T> {
        match self {
            KeyActionSet::Single(a1) => KeyActionSet::Single(a1.invert()),
            KeyActionSet::Double(a1, a2) =>  KeyActionSet::Double(a1.invert(), a2.invert()),
            KeyActionSet::Triple(a1, a2, a3) => KeyActionSet::Triple(a1.invert(), a2.invert(), a3.invert()),
        }
    }
}

impl<T> Default for KeyActionSet<T> {
    fn default() -> Self {
        Self::Single(KeyAction::default())
    }
}


#[derive(Debug, Clone, Copy)]
pub enum KeyConf<T> {
    Tap(TapKeyConf<T>),
    Hold(HoldKeyConf<T>),
    DoubleTap(DoubleTapKeyConf<T>),
    DoubleTapHold(DoubleTapHoldKeyConf<T>),
}

impl<T> Default for KeyConf<T> {
    fn default() -> Self {
        todo!()
    }
}

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
