mod epoll;
pub mod monitor;
pub mod virtual_dev;

use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::fmt;
use std::io::Error as IOError;
use std::time::SystemTimeError;
use std::error;

use kb_core::keyboard::Event;
use kb_core::keyboard::Action;
use kb_core::keyboard::Keyboard;
use evdev_rs::enums::{EV_KEY};

use monitor::EventIter;
use epoll::Epoll;
use virtual_dev::UInputKeyboard;


#[derive(Debug)]
pub enum Error {
    IO(IOError),
    Time(SystemTimeError),
    DeviceInit,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(io_err) => write!(f, "io err: {}", io_err),
            Error::Time(time_err) => write!(f, "error creating input event: {}", time_err),
            Error::DeviceInit => write!(f, "Error initializing uinput device")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IO(err) => Some(err),
            Error::Time(err) => Some(err),
            _ => None
        }
    }
}

impl From<IOError> for Error {
    fn from(io_error: IOError) -> Error {
        Error::IO(io_error)
    }
}

impl From<SystemTimeError> for Error {
    fn from(sys_time_err: SystemTimeError) -> Error {
        Error::Time(sys_time_err)
    }
}


type Result<T> = std::result::Result<T, Error>;


pub struct Runtime {
    emitter: EventIter,
    virtual_dev: UInputKeyboard,
    keyboard: Box<dyn Keyboard<EV_KEY, EV_KEY>>,
    epoll: Epoll
}

impl Runtime {
    pub fn new(emitter: EventIter, virtual_dev: UInputKeyboard, keyboard: impl Keyboard<EV_KEY, EV_KEY> + 'static, poll_period: Duration) -> Result<Self> {
        let mut epoll = Epoll::new(10, poll_period)?;
        epoll.monitor_file(&emitter)?;
        
        Ok(Self {
            emitter: emitter,
            virtual_dev: virtual_dev,
            keyboard: Box::new(keyboard),
            epoll: epoll
        })
    }

    pub fn run(&mut self) {
        loop {
            let mut poll = false;
            {
                let mut ready_iter = self.epoll.wait().unwrap();
                poll = matches!(ready_iter.next(), None);
            }
            self.emit_event(poll).unwrap();
        }
    }

    fn emit_event(&mut self, poll: bool) -> Result<()> {
        let event = if poll { Event::Poll } else { self.emitter.next().unwrap_or(Event::Poll) };
        let actions = self.keyboard.transition(event);
        self.virtual_dev.emit_events(&actions)
    }
}
