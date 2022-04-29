use crate::keyboard::Keyboard;
use crate::keyboard::Action;
use crate::keyboard::Event;

/// Sample implementation of Keyboard trait that echoes
/// the input event as an action
pub struct EchoerKb { }

impl<T> Keyboard<T, T> for EchoerKb {
    fn transition(&mut self, event: Event<T>) -> Vec<Action<T>> {
        match event {
            Event::KeyPress(code) => vec![Action::SendCode(code)],
            Event::KeyRelease(code) => vec![Action::Stop(code)],
            Event::Poll => Vec::new()
        }
    }
}
