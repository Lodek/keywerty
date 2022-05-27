//! SMKeyboard usage example showing illustrating how to create custom
//! key configurations for composed behaviors.
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use keywerty::keyboard::Action;
use keywerty::keyboard::Event;
use keywerty::keyboard::Keyboard;
use keywerty::keyboard::SMKeyboard;
use keywerty::keyboard::SMKeyboardSettings;
use keywerty::keys;
use keywerty::mapper::LayerId;
use keywerty::mapper::LayerMapper;

const default_layer: u8 = 0;

fn main() {
    let mapper = build_mapper();
    let settings = SMKeyboardSettings::default();
    let mut keyboard = SMKeyboard::new(default_layer, mapper, settings);

    println!("Press Tap key");
    let actions = keyboard.transition(Event::KeyPress(0));
    print_actions(&actions);

    println!("Released");
    let actions = keyboard.transition(Event::KeyRelease(0));
    print_actions(&actions);

    println!("Activate layer");
    let actions = keyboard.transition(Event::KeyPress(2));
    print_actions(&actions);

    println!("Press Tap Key in layer");
    let actions = keyboard.transition(Event::KeyPress(0));
    print_actions(&actions);

    println!("Released");
    let actions = keyboard.transition(Event::KeyRelease(0));
    print_actions(&actions);

    println!("Released layer");
    let actions = keyboard.transition(Event::KeyRelease(2));
    print_actions(&actions);

    // Hold keys are a bit more intricate because they require a timing aspect.
    // Internally the hold key is handled by a state machine which sometimes
    // require a Poll event in order to transition to future states.
    //
    // In a real runtime, it's recommended to use a loop and poll the
    // keyboard after a certain threshold to make this process transparent
    // to the end user.
    println!("Press hold key and wait until its active");

    // the initial press doesn't do anything as it needs to wait for timeout
    keyboard.transition(Event::KeyPress(1));

    // after timeout, the keyboard will emit the hold key
    thread::sleep(Duration::from_millis(800));
    let actions = keyboard.transition(Event::Poll);
    print_actions(&actions);

    println!("Releasing the held key works as expected");
    let actions = keyboard.transition(Event::KeyRelease(1));
    print_actions(&actions);

    // Likewise, the tap behavior of a hold key requires a few polling events.
    println!("Tap the Hold key");

    // again, the initial press doesn't do anything
    keyboard.transition(Event::KeyPress(1));

    println!("Release the Hold key trigger the tap action");
    // after releasing the key, the keyboard will emit the tap key
    let actions = keyboard.transition(Event::KeyRelease(1));
    print_actions(&actions);

    println!("Once the keyboard is polled, the key is released");
    let actions = keyboard.transition(Event::Poll);
    print_actions(&actions);
}

/// Builds mapper with custom key actions
/// Demonstrates how to configure a keyboard using different
/// key configurations
fn build_mapper() -> impl LayerMapper<u8, String> {
    let mut map = HashMap::new();

    // Map key 0 to a simple Tap action sending 0.
    // KeyConf indicate the key behavior and the action
    // it should take.
    let action = keys::KeyAction::SendKey(String::from("key 0 tapped in layer 0"));
    let conf = keys::TapKeyConf { tap: action.into() };
    map.insert((default_layer, 0), keys::KeyConf::Tap(conf));

    // map key 1 as a Hold key, performing one action when held, another when pressed.
    let tap_action = keys::KeyAction::SendKey(String::from("key 1 tapped"));
    let hold_action = keys::KeyAction::SendKey(String::from("key 1 held"));
    let conf = keys::HoldKeyConf {
        tap: tap_action.into(),
        hold: hold_action.into(),
    };
    map.insert((default_layer, 1), keys::KeyConf::Hold(conf));

    // maps key 2 to activate layer 1
    let action = keys::KeyAction::PushLayer(1);
    let conf = keys::TapKeyConf { tap: action.into() };
    map.insert((default_layer, 2), keys::KeyConf::Tap(conf));

    // maps key 0 in layer 1 to a tap action
    let action = keys::KeyAction::SendKey(String::from("key 0 tapped in layer 1"));
    let conf = keys::TapKeyConf { tap: action.into() };
    map.insert((1, 0), keys::KeyConf::Tap(conf));

    map
}

/// Print actions in result vector in debug mode
fn print_actions(actions: &Vec<Action<String>>) {
    for action in actions.iter() {
        println!("received action: {:?}", action);
    }
}
