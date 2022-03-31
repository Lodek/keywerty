use crate::virtual_kb::{Result, Error};

enum Error {
    EpollCreate(c_int),
    EpollMonitor(c_int),
    EpollWait(c_int),
}

/// Listen to an event device and generates reports
/// Event loop which should run in its own thread.
/// Receive writting end of channel to send event report
/// spawn new thrad
pub struct EvdevListener {

}

impl EvdevListener {

    pub fn new(device: Path) -> Result<Self> {
        let mut file = File::open(device)?;
        let mut epoll = Epoll::new(10)?;
        epoll.monitor_file(file.as_raw_fd())?;
        // TODO so there's a little situation here that I want to handle regarding the device
        // file.
        // I want this to be a safe implementation and right now it's not.
        // Epoll uses the dev file, and I extracted its fd to use with epoll.
        // If the dev file is somehow closed, we done for chief.
        // or rather, epoll should remove internally, no?
        // anyway, i am trying to think of a constraint that makes this safe-ish, such that
        // the file is bound to the epoll instance.
        let mut report_iter = EvdevReportIter::new(epoll);
        Ok(Self {
            epoll,
            file,
            report_iter
        })
    }

    pub fn run(tx: Sender<RuntimeEvent>)  {
        loop {
            // TODO what should I do in case of an error?
            // Ignore it?
            // log to stdout?
            // kill thread?
            self.epoll.wait().unwrap();
            self.report_iter.next().map(|report_res| {
                match report_res {
                    Ok(report) => {
                        tx.send(report).unwrap();
                    },
                    err => {
                    }
                }
            });
        }
    }
}



/// Safe interface around Linux's epoll.
/// Allows creating an epoll kernel instance
/// and monitoring a file for read events.
struct Epoll  {
    epoll_fd: RawFd,
    event_buff: Vec<epoll_event>,
}

// TODO what if I convert this to an iterator?
// it's completely unecessary but might be cool.
impl Epoll {

    /// Create new epoll instance
    fn new(event_buff_size: usize) -> Result<Self> {
        unsafe {
            let fd = epoll_create1(0);
            if fd >= 1 {
                let epoll = Self {
                    epoll_fd: fd,
                    event_buff: Vec::with_capacity(event_buff_size)
                }
                Ok(epoll)
            }
            else {
                let errno = get_errno()
                Err(Error::EpollCreate(errno))
            }
        }
    }
         
    /// Add file to Epoll's interest list.
    /// The file shall only be monitored for EPOLLIN (ie read will not block).
    /// Monitoring a previously added file will cause an error.
    fn monitor_file(&mut self, fd: RawFd) -> Result<()> {
        let event = epoll_event {
            events: EPOLLPRI,
            u64: fd
        }

        unsafe {
            let result = epoll_ctl(self.epoll_fd, EPOLL_CTL_ADD, fd, &mut event as *mut _);
            if result < 0 {
                Err(Error::EpollMonitor(get_errno()))
            }
            else {
                Ok(())
            }
        }
    }

    /// Perform an indefinite wait over the list of registered files.
    /// `wait` blocks until any registered file is ready to read.
    ///
    /// Return slice with file descriptors matching the ready files.
    fn wait(&mut self) -> Result<impl Iterator<Type=RawFd>> {
        unsafe {
            let never_timeout = -1;
            let event_count = epoll_wait(self.epoll_fd, self.event_buff.as_mut_ptr(), self.event_buff.capacity(), never_timeout);
            if event_count < 0 {
                Err(Error::EpollWait(get_errno()))
            }
            else {
                // because the ffi call writes to the buffer without updating
                // `Vec`'s internal state, we neeed to reconstruct the vector
                // using the result of `epoll_wait`.
                // The `*_raw_parts` method family in rust allows us to
                // take ownership over the internal memory buffer and reconstruct it.
                let (buff, _, capacity) = self.event_buff.into_raw_parts();
                self.event_buff = Vec::from_raw_parts(buff, event_count, capacity);
                Ok(self.event_buff.iter().map(|event| *event.u64 as RawFd))
            }
        }
    }
}

impl Drop for Epoll {
    /// Close the previously created epoll instance.
    /// Panics if `close` fails.
    fn drop(&mut self) {
        unsafe {
            let result = libc::close(self.epoll_fd);
            if result < 0 {
                panic!("Error closing Epoll instance: {}", get_errno())
            }
        }
    }
}

/// Iterator that returns an Evdev event for a give device file.
/// Calling `next` will perform a device read, which in turn will
/// return an event.
/// 
/// On its own, `next` will be a blocking call, as such this should be
/// paired with `Epoll` to build a non_blocking event loop
struct EventIter {
    device: Device
}

impl EventIter {
    // Should I take a Device or a File?
    pub fn new(file: File) -> Result<Self> {
        let device = Device::new_from_file(file)?;
        Ok(Self {
           device
       })
    }
}

impl Iterator for EventIter {
    type Item = Result<EV_KEY>;
    fn next(&mut self) -> Option<EV_KEY> {
        // FIXME this implementation completely ignores the SYN_DROPPED
        // events from evdev and must be revisted.
        // See:
        // - https://www.freedesktop.org/software/libevdev/doc/latest/syn_dropped.html
        // - https://docs.rs/evdev-rs/latest/evdev_rs/struct.Device.html#method.next_event
        self.device.next_event(evdev::NORMAL).map(|(_, event) event)
    }
}


struct EvdevReportIter {
    event_iter: EventIter,
    report_buffer: EventReport,
    has_lost_event: bool,
}

/// Iterator for EvdevReports.
/// Aggregates Evdev Events into a report.
/// Evedev events are combined to create a report and any SYN_DROPPED
/// events are handled transparently.
///
/// EvdevReportIter behavior is similar to [`TryIter`](https://doc.rust-lang.org/std/sync/mpsc/struct.TryIter.html) from the `mpsc` module.
/// It will return None while the read events don't make up a full Report.
/// Once a report has been succesfully built, the iterator will return a Result.
/// 
/// This iterator may block if the evdev device file isn't ready, therefore it should
/// be paired with `Epoll` to build the event loop.
///
/// The caller should continously call the iterator until a result is build.
impl EvdevReportIter {
    fn new(event_iter: EventIter) -> Self {
        Self {
            report_buffer: Vec::new(),
            has_lost_event: false,
            event_iter,
        }
    }
}

impl Iterator for EvdevReportIter {
    type Item = Result<EventReport>;

    fn next(&mut self) -> Option<EventReport> {
        // FIXME fix this implementation to handle SYN_DROPPED
        
        // event_iter should be an infinite iterator therefore unwrap is safe
        match self.event_iter.next().unwrap() {
            Ok(event) => {
                self.report_buffer.push(event.event_code);
                match event.event_code {
                     EventCode::EV_SYN(SYN_REPORT) => {
                         let buff = self.report_buffer;
                         self.report_buffer = Vec::new();
                         Ok(buff)
                     },
                     event => None
                }
            },
            err => Some(err)
        }
    }
}
