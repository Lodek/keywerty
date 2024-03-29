//! Sample program demosntrating vkwrty runtime usage and functionality
//! Employs a simple "echoer" keyboard, which echoes the input as the output.
//!
//! This keyboard model works in Linux because the intercepted uinput event
//! will be the same as the emitted one.

use std::time::Duration;
use std::ffi::CString;
use std::os::unix::io::FromRawFd;
use std::fs;

use keywerty::keyboard::Action;
use keywerty::keyboard::Event;
use keywerty::keyboard::Keyboard;
use clap::Arg;
use clap::App;

use vkwrty::Error;
use vkwrty::Runtime;
use vkwrty::virtual_dev::UInputKeyboard;
use vkwrty::monitor::EventIter;


/// Implementation of Keyboard trait that echoes the input event data as an action.
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


fn main() {
    let matches = App::new("Virtual echoer Keyboard")
        .arg(Arg::with_name("event source")
             .required(true)
             .value_name("EV_FILE")
             .help("Linux input file from which events should be listened")
             .takes_value(true))
        .get_matches();

    let ev_file = matches.value_of("event source").unwrap();
    let ev_file = open_dev(&ev_file);

    let event_iter = EventIter::new(ev_file).unwrap();

    let virtual_dev = UInputKeyboard::new(&"Echoer keyboard").unwrap();

    let keyboard = EchoerKb {};

    let mut runtime = Runtime::new(event_iter, virtual_dev, keyboard, Duration::from_secs(300)).unwrap();
    runtime.run()
}

fn open_dev(path: &str) -> fs::File {
    unsafe {
        let flags = libc::O_NONBLOCK | libc::O_RDONLY;
        let path = CString::new(path).unwrap();
        let fd = libc::open(path.as_ptr(), flags);
        fs::File::from_raw_fd(fd)
    }
}
