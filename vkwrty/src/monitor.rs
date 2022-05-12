use std::io;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::os::unix::prelude::RawFd;

use evdev_rs::ReadFlag;
use evdev_rs::Device;
use evdev_rs::InputEvent;
use evdev_rs::enums::EventCode;
use evdev_rs::enums::EV_KEY;
use keywerty::keyboard::Event;


/// Iterator that returns an Evdev event for a give device file.
/// Calling `next` will perform a device read, which in turn will
/// return an event.
pub struct EventIter {
    device: Device,
    events: Vec<Event<EV_KEY>>
}

impl AsRawFd for EventIter {
    fn as_raw_fd(&self) -> RawFd {
        self.device.file().as_raw_fd()
    }
}

// TODO use log facade for debug log
// https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html
// https://docs.rs/log/latest/log/
impl EventIter {

    pub fn new(file: File) -> io::Result<Self> {
        let device = Device::new_from_file(file)?;
        
        // FIXME grab that from the linux header. figure out how to do that through rust
        let EVIOCGRAB = 1074021776;

        unsafe {
            let fd = device.file().as_raw_fd();
            let rv = libc::ioctl(fd, EVIOCGRAB, 1);
            if rv == -1 {
                return Err(io::Error::last_os_error());
            }
        }

        Ok(Self {
           device,
           events: Vec::new()
       })
    }

    fn read_all_events(&mut self) {
        // FIXME this implementation completely ignores the SYN_DROPPED
        // events from evdev and must be revisted.
        // See:
        // - https://www.freedesktop.org/software/libevdev/doc/latest/syn_dropped.html
        // - https://docs.rs/evdev-rs/latest/evdev_rs/struct.Device.html#method.next_event
        loop {
            match self.device.next_event(ReadFlag::NORMAL) {
                Ok((_, input_event)) => {
                    eprintln!("read event: {:?}", input_event);
                    if let Some(event) = self.map_event(input_event) {
                        self.events.push(event);
                    }
                },
                Err(error) => {
                    eprintln!("error reading event device: {:?}", error);
                    return;
                }
            }
        }
    }

    fn map_event(&mut self, input_event: InputEvent) -> Option<Event<EV_KEY>> {
        match &input_event {
            InputEvent { event_code: EventCode::EV_KEY(ev_key), value: 0, .. } => Some(Event::KeyRelease(*ev_key)),
            InputEvent { event_code: EventCode::EV_KEY(ev_key), value: 1, .. } => Some(Event::KeyPress(*ev_key)),
            ev => {
                eprintln!("dropped input event: {:?}", input_event);
                None
            }
        }
    }
}

impl Iterator for EventIter {
    type Item = Event<EV_KEY>;

    /// Return the next event from the event queue.
    /// If the internal queue is empty, perform a device read.
    ///
    /// Note that reading the device will not block, therefore it should be paired
    /// epoll to avoid busy looping.
    fn next(&mut self) -> Option<Self::Item> {
        if self.events.is_empty() {
            self.read_all_events();
        }
        self.events.pop()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_iterator_return_elements() {
        // TODO research how to mock `Device`
        // or crate trait and implement for `Device`
        // such that I can use a stub interface.
    }

}
