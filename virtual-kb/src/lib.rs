mod epoll;
mod monitor;

use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use kb_core::keyboard::Event;
use kb_core::keyboard::Action;
use kb_core::mapper::HashMapMapper;

struct KeyboardConfiguration {
    
}

struct Configuration {
    kb_poll_period: Duration,
    virtual_device_name: String,
    kb_config: KeyboardConfiguration,
}

//fn parse_map(map: &str) -> Result<HashMapMapper> {
    //todo!()
//}
