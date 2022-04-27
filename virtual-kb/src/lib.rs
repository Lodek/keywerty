mod epoll;
mod monitor;
mod virtual_dev;

use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use kb_core::keyboard::Event;
use kb_core::keyboard::Action;
use kb_core::keyboard::Keyboard;
use kb_core::mapper::HashMapMapper;
use evdev_rs::enums::{EV_KEY};

use monitor::EventIter;
use epoll::Epoll;
use virtual_dev::UInputKeyboard;

struct Runtime {
    emitter: EventIter,
    virtual_dev: UInputKeyboard,
    keyboard: Box<dyn Keyboard<EV_KEY, EV_KEY>>,
    epoll: Epoll
}

impl Runtime {
    pub fn new(emitter: EventIter, virtual_dev: UInputKeyboard, keyboard: impl Keyboard<EV_KEY, EV_KEY>, poll_period: Duration) -> Result<Self> {
        let mut epoll = Epoll::new(10, poll_period)?;
        // TODO impl AsRawFd for EventIter
        epoll.monitor_file(&virtual_dev)?;
        
        Ok(Self {
            emitter: emitter,
            virtual_dev: virtual_dev,
            keyboard: Box::new(keyboard),
            epoll: epoll
        })
    }

    pub fn runtime(&mut self) {
        loop {
            let ready_iter = self.epoll.wait().unwrap();
            let poll = matches!(ready_iter.next(), None);
            self.emit_event().unwrap();
        }
    }

    fn emit_event(&mut self, poll: bool) -> Result<()> {
        let event = if poll { Event::Poll } else { self.emitter.next().unwrap_or_default() };
        let actions = self.keyboard.transition(event);
        self.virtual_dev.emit_event(&actions)
    }
}
