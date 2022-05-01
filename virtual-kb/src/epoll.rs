/// Safe epoll wrap limited to EPOLLIN events

use std::os::unix::io::AsRawFd;
use std::os::unix::prelude::RawFd;
use std::time::Duration;
use std::any::Any;
use std::io::{Result, Error};

use libc;
use libc::c_int;
use num_traits::AsPrimitive;


/// Safe interface around Linux's epoll.
/// Allows creating an epoll kernel instance
/// and monitoring a file for read events.
pub struct Epoll  {
    epoll_fd: RawFd,
    event_buff: Vec<libc::epoll_event>,
    read_timeout: Duration,
}

impl Epoll {

    /// Create new epoll instance
    pub fn new(event_buff_size: usize, read_timeout: Duration) -> Result<Self> {
        unsafe {
            let fd = libc::epoll_create1(0);
            if fd >= 1 {
                let epoll = Self {
                    read_timeout,
                    epoll_fd: fd,
                    event_buff: Vec::with_capacity(event_buff_size),
                };
                Ok(epoll)
            }
            else {
                Err(Error::last_os_error())
            }
        }
    }
         
    /// Add file to Epoll's interest list.
    /// Epoll takes ownership over the file and returns a mutable reference to it.
    /// For the purpose of this implementation, a file is something that implements
    /// the `AsRawFd` trait.
    ///
    /// The file shall only be monitored for EPOLLIN (ie read will not block).
    /// Monitoring a previously added file will cause an error.
    pub fn monitor_file<F>(&mut self, file: &F) -> Result<()>
    where F: AsRawFd
    {
        let fd = file.as_raw_fd();
        let mut event = libc::epoll_event {
            events: (libc::EPOLLPRI | libc::EPOLLIN) as u32,
            u64: fd as u64
        };

        unsafe {
            let result = libc::epoll_ctl(self.epoll_fd, libc::EPOLL_CTL_ADD, fd, &mut event as *mut _);
            if result < 0 {
                Err(Error::last_os_error())
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
    pub fn wait(&mut self) -> Result<impl Iterator<Item=RawFd> + '_> {
        unsafe {
            // epoll timeout expects a number of milliseconds
            let timeout: c_int = self.read_timeout.as_millis().as_();
            let event_count = libc::epoll_wait(self.epoll_fd, self.event_buff.as_mut_ptr(), self.event_buff.capacity().as_(), timeout);
            eprintln!("epoll_wait result: {}", event_count);
            if event_count < 0 {
                Err(Error::last_os_error())
            }
            else {
                self.event_buff.set_len(event_count as usize);
                Ok(self.event_buff.iter().map(|event| event.u64 as RawFd))
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
                panic!("Error closing Epoll instance: {}", Error::last_os_error())
            }
        }
    }
}
