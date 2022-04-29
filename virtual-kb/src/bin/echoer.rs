use std::time::Duration;
use std::fs;

use kb_core::keyboard::Action;
use kb_core::keyboard::Event;
use kb_core::keyboard::Keyboard;
use kb_core::keyboard::echoer::EchoerKb;
use clap::Arg;
use clap::App;

use virtual_kb::Error;
use virtual_kb::Runtime;
use virtual_kb::virtual_dev::UInputKeyboard;
use virtual_kb::monitor::EventIter;

fn main() {
    let matches = App::new("Virtual echoer Keyboard")
        .arg(Arg::with_name("event source")
             .required(true)
             .value_name("EV_FILE")
             .help("Linux input file from which events should be listened")
             .takes_value(true))
        .get_matches();

    let ev_file = matches.value_of("event source").unwrap();
    let ev_file = fs::OpenOptions::new()
        .read(true)
        .write(false)
        .open(ev_file)
        .unwrap();

    let event_iter = EventIter::new(ev_file).unwrap();

    let virtual_dev = UInputKeyboard::new(&"Echoer keyboard").unwrap();

    let keyboard = EchoerKb {};

    let mut runtime = Runtime::new(event_iter, virtual_dev, keyboard, Duration::from_millis(100)).unwrap();
    runtime.run()
}
