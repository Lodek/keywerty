//! Main runtime for vkwerty

use vkwrty::Runtime;
use vkwrty::Error;
use vkwrty::monitor::EventIter;
use vkwrty::virtual_dev::UInputKeyboard;
use vkwrty::open_dev;

use std::collections::HashMap;
use std::time::Duration;

use keywerty::mapper::MapOrEchoMapper;
use keywerty::keyboard::SMKeyboard;
use keywerty::keyboard::SMKeyboardSettings;
use keywerty::keyboard::Action;
use keywerty::keyboard::Event;
use keywerty::keys;
use keywerty::keyboard::Keyboard;
use clap::App;
use clap::Arg;
use evdev_rs::enums::EV_KEY;


fn main() {

    let matches = App::new("Virtual Keyboard")
        .arg(Arg::with_name("event source")
             .required(true)
             .value_name("EV_FILE")
             .help("Linux input file from which events should be listened")
             .takes_value(true))
        .get_matches();

    let ev_file = matches.value_of("event source").unwrap();
    let ev_file = open_dev(&ev_file);

    let event_iter = EventIter::new(ev_file).unwrap();

    let virtual_dev = UInputKeyboard::new(&"Virtual keyboard").unwrap();

    let settings = SMKeyboardSettings::default();
    let mapper = build_mapper();
    let keyboard = SMKeyboard::new(0, mapper, settings);

    let mut runtime = Runtime::new(event_iter, virtual_dev, keyboard, Duration::from_millis(100)).unwrap();
    runtime.run()
}
