/// Represents a virtual uinput device.
/// Initializes device and emits events

use std::io::Error as IOError;
use std::time::{SystemTime, SystemTimeError};
use std::iter::once;

use evdev_rs::{TimeVal, UInputDevice, InputEvent, UninitDevice, DeviceWrapper};
use evdev_rs::enums::{EV_SYN, EV_KEY, EventType, EventCode, int_to_ev_key};


#[derive(Debug)]
pub enum Error {
    IO(IOError),
    Time(SystemTimeError),
    DeviceInit,
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


/// Models a Virtual Linux device, based on the kernel's `uinput` module.
/// The virtual device is used to emit IO Events, allowing us to create "virtual" keyboard / mouses
/// and etc.
///
/// The virtual device in this module is configured to emit only a subset of IO Events, namely
/// events related to key presses.
pub struct UInputKeyboard {
    dev: UInputDevice
}

impl UInputKeyboard {

    /// Create a new uinput device with the given name.
    /// The device is enable to send every key event as 
    /// defined in linux's `input-event-codes.h` header.
    pub fn new<'a>(name: &str) -> Result<Self> {
        let mut dev = UninitDevice::new().ok_or(Error::DeviceInit)?;
        dev.set_name(name);
        dev.enable(&EventType::EV_KEY)?;

        Self::get_ev_keys()
            .map(|key_code| EventCode::EV_KEY(key_code))
            .map(|event_code| dev.enable(&event_code))
            .fold(Ok(()), |acc, result| acc.and(result))?;
                
        let uinput_dev = UInputDevice::create_from_device(&dev)?;
        Ok(Self { dev: uinput_dev })
    }

    /// Build and emit a report to the underlyin `uinput` device.
    ///
    /// Reports are chain of events terminated with a `SYN_REPORT` event.
    pub fn emit_events(&mut self, pairs: &[(EV_KEY, i32)]) -> Result<()> {
        let timeval = Self::build_timeval();

        // According to the examples in the docs, `SYN_REPORT` events should
        // have 0 as the value.
        // See:
        // - https://www.kernel.org/doc/html/latest/input/uinput.html#keyboard-events
        // - https://www.freedesktop.org/software/libevdev/doc/latest/group__uinput.html#ga4c3c2f5fcd315a28a067f53b9f855fe7
        let report_eventcode = EventCode::EV_SYN(EV_SYN::SYN_REPORT);
        let report_event = InputEvent::new(&timeval, &report_eventcode, 0);

        pairs.iter()
            .map(|(key_code, value)| InputEvent::new(&timeval, &EventCode::EV_KEY(*key_code), *value))
            .chain(once(report_event))
            .map(|input_event| self.dev.write_event(&input_event))
            .fold(Ok(()), |acc, result| acc.and(result))
            .map_err(|e| e.into())
    }

    /// Return an evdev `TimeVal` for the current instant
    fn build_timeval() -> TimeVal {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let secs = now.as_secs().try_into().unwrap();
        let micros = now.as_micros().try_into().unwrap();
        TimeVal::new(secs, micros)
    }

    /// Return iterator with every EV_KEY variant
    fn get_ev_keys() -> impl Iterator<Item=EV_KEY> {
        // This is kinda bad but...
        // For some reason the evdev crate does not provide a method that returns
        // all variants for `EV_KEY`
        // EV_KEY events apparently range from 0 to ~750.
        //
        // Anyway, so instead copying the definition and goin line by line we can waste a few CPU
        // cycles here and not repeat the whole thing
        (0..1000)
            .map(int_to_ev_key)
            .filter(|res| res.is_some())
            .map(|res| res.unwrap())
    }
}
