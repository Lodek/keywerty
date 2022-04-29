use std::io;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::os::unix::prelude::RawFd;

use evdev_rs::ReadFlag;
use evdev_rs::Device;
use evdev_rs::InputEvent;
use evdev_rs::enums::EventCode;
use evdev_rs::enums::EV_KEY;
use kb_core::keyboard::Event;

/// Iterator that returns an Evdev event for a give device file.
/// Calling `next` will perform a device read, which in turn will
/// return an event.
/// 
/// On its own, `next` will be a blocking call, as such this should be
/// paired with `Epoll` to build a non_blocking event loop
pub struct EventIter {
    device: Device
}

impl AsRawFd for EventIter {
    fn as_raw_fd(&self) -> RawFd {
        self.device.file().as_raw_fd()
    }
}

impl EventIter {
    // Should I take a Device or a File?
    pub fn new(file: File) -> io::Result<Self> {
        let device = Device::new_from_file(file)?;
        Ok(Self {
           device
       })
    }
}

impl Iterator for EventIter {
    type Item = Event<EV_KEY>;

    fn next(&mut self) -> Option<Self::Item> {
        // FIXME this implementation completely ignores the SYN_DROPPED
        // events from evdev and must be revisted.
        // See:
        // - https://www.freedesktop.org/software/libevdev/doc/latest/syn_dropped.html
        // - https://docs.rs/evdev-rs/latest/evdev_rs/struct.Device.html#method.next_event
        match self.device.next_event(ReadFlag::NORMAL) {
            Ok((_, InputEvent { event_code: EventCode::EV_KEY(ev_key), value: 0, ..})) => Some(Event::KeyRelease(ev_key)),
            Ok((_, InputEvent { event_code: EventCode::EV_KEY(ev_key), value: 1, ..})) => Some(Event::KeyPress(ev_key)),
            Ok(ok) => {
                // TODO debug log with skipped value
                // https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html
                // https://docs.rs/log/latest/log/
                None
            },
            Err(err) => {
                // TODO err log with err info
                None
            },
        }
    }
}
