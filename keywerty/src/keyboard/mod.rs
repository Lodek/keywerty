/// Module defines a Logical keyboard and its dependent types.
///
/// The logical keyboard interface was drawn out considering 
/// types which match an USB HID keyboard, that is, key scan codes are 1 byte.

mod smkb;
mod echoer;

pub use smkb::SMKeyboardSettings;
pub use smkb::SMKeyboard;
pub use echoer::EchoerKb;


/// Set of events that a keyboard respond to. (inputs)
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Event<Id> {
    KeyPress(Id),
    KeyRelease(Id),
    Poll,
}

impl<Id> Event<Id> {
    pub fn is_key_press(&self) -> bool {
        matches!(self, Event::KeyPress(_))
    }

    pub fn is_key_release(&self) -> bool {
        matches!(self, Event::KeyRelease(_))
    }

    pub fn get_key_id(&self) -> Option<&Id> {
        match self {
            Event::KeyPress(key_id) => Some(key_id),
            Event::KeyRelease(key_id) => Some(key_id),
            Event::Poll => None,
        }
    }
}


/// Set of actions a keyboard perform as consequence of inputs. (outputs)
#[derive(Debug, Clone, PartialEq)]
pub enum Action<T> {
    SendCode(T),
    Stop(T)
}


/// Abstraction for a physical keyboard.
/// Conceptually a keyboard contains keys, each identified with an id.
///
/// The keyboard receives `Event<Id>`s as input and returns a set
/// of `Action`s indicating what should be done.
///
/// It can be thought of as a state machine, each time it receives an input
/// it goes to a different state and produces an output
pub trait Keyboard<KeyId, T> {
    fn transition(&mut self, event: Event<KeyId>) -> Vec<Action<T>>;
}
