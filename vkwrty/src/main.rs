use vkwrty::Runtime;
use vkwrty::Error;
use vkwrty::monitor::EventIter;
use vkwrty::virtual_dev::UInputKeyboard;

use std::collections::HashMap;
use std::time::Duration;
use std::fs;
use std::ffi::CString;
use std::os::unix::io::FromRawFd;

use keywerty::mapper::MapOrEchoMapper;
use keywerty::keyboard::r#impl as sm_kb;
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

    let settings = sm_kb::SMKeyboardSettings::default();
    let mapper = build_mapper();
    let keyboard = sm_kb::SMKeyboard::new(0, mapper, settings);

    let mut runtime = Runtime::new(event_iter, virtual_dev, keyboard, Duration::from_millis(100)).unwrap();
    runtime.run()
}

fn build_mapper() -> MapOrEchoMapper<EV_KEY> {
    let mut map = HashMap::new();

    // caps lock on tap, control on hold
    map.insert((0, EV_KEY::KEY_CAPSLOCK),
        keys::KeyConf::Hold(
            keys::HoldKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_ESC)),
                hold: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_LEFTCTRL)),
        })
    );

    map.insert((0, EV_KEY::KEY_LEFTCTRL),
        keys::KeyConf::Tap(
            keys::TapKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::PushLayer(1)),
        })
    );

    map.insert((1, EV_KEY::KEY_J),
        keys::KeyConf::Tap(
            keys::TapKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_DOWN)),
        })
    );

    map.insert((1, EV_KEY::KEY_K),
        keys::KeyConf::Tap(
            keys::TapKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_UP)),
        })
    );

    map.insert((1, EV_KEY::KEY_L),
        keys::KeyConf::Tap(
            keys::TapKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_RIGHT)),
        })
    );

    map.insert((1, EV_KEY::KEY_H),
        keys::KeyConf::Tap(
            keys::TapKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_LEFT)),
        })
    );

    MapOrEchoMapper(map)
}

fn open_dev(path: &str) -> fs::File {
    unsafe {
        let flags = libc::O_NONBLOCK | libc::O_RDONLY;
        let path = CString::new(path).unwrap();
        let fd = libc::open(path.as_ptr(), flags);
        fs::File::from_raw_fd(fd)
    }
}
