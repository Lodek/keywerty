//! Module with declaration of a LayerMapper implementation for my custom
//! keyboard configuartion.

use std::collections::HashMap;

use keywerty::mapper::MapOrEchoMapper;
use keywerty::mapper::LayerId;
use evdev_rs::enums::EV_KEY;
use keywerty::keys;


const LAYER_DEFAULT: u8 = 0;
const LAYER_CTRL: u8 = 0;

/// Construct keyboard configuration map
pub fn build_mapper() -> MapOrEchoMapper<EV_KEY> {
    let mut map = HashMap::new();

    // caps lock key sends esc on tap, control on hold
    map.insert((LAYER_DEFAULT, EV_KEY::KEY_CAPSLOCK),
        keys::KeyConf::EagerHold(
            keys::HoldKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_ESC)),
                hold: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_LEFTCTRL)),
        })
    );

    // Enable Ctrl Layer
    map.insert((LAYER_DEFAULT, EV_KEY::KEY_LEFTCTRL),
        keys::KeyConf::Tap(
            keys::TapKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::PushLayer(LAYER_CTRL)),
        })
    );


    // disable esc to get used to ctrl
    map.insert((LAYER_DEFAULT, EV_KEY::KEY_ESC),
        keys::KeyConf::Tap(
            keys::TapKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::NoOp),
        })
    );

    set_vim_arrow_keys_in_layer(&mut map, LAYER_CTRL);

    MapOrEchoMapper(map)
}


pub fn set_vim_arrow_keys_in_layer(map: &mut HashMap<(LayerId, EV_KEY), keys::KeyConf<EV_KEY>>, layer: LayerId) {
    map.insert((layer, EV_KEY::KEY_J),
        keys::KeyConf::Tap(
            keys::TapKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_DOWN)),
        })
    );

    map.insert((layer, EV_KEY::KEY_K),
        keys::KeyConf::Tap(
            keys::TapKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_UP)),
        })
    );

    map.insert((layer, EV_KEY::KEY_L),
        keys::KeyConf::Tap(
            keys::TapKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_RIGHT)),
        })
    );

    map.insert((layer, EV_KEY::KEY_H),
        keys::KeyConf::Tap(
            keys::TapKeyConf { 
                tap: keys::KeyActionSet::Single(keys::KeyAction::SendKey(EV_KEY::KEY_LEFT)),
        })
    );
}
