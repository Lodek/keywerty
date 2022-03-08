/// Keyboard module with main abstractions that rule a keyboard
pub mod map;

/// HID Scan codes
// TODO convert this to an enum cause pretty
pub type KeyCode = u8;

pub type KeyId = u8;

/// Set of events that a keyboard respond to. (inputs)
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Event {
    KeyPress(KeyId),
    KeyRelease(KeyId),
    Poll,
}

impl Event {
    pub fn is_key_press(&self) -> bool {
        match self {
            Event::KeyPress(_) => true,
            _ => false
        }
    }

    pub fn get_key_id(&self) -> KeyId {
        match self {
            Event::KeyPress(key_id) => *key_id,
            Event::KeyRelease(key_id) => *key_id,
            Event::Poll => 0,
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
    fn transition<'a>(&mut self, event: Event) -> Vec<Action>;
}
