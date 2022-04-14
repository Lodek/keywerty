/// Module introduces types used for layers and a mapper trait
use crate::keys::{KeyCode, KeyId, LayerId, KeyConf, KeyAction, TapKeyConf, KeyActionSet};

use std::collections::HashMap;



/// Trait to ease mapping handling keyboard configurations
/// when multiple layers are supported.
pub trait LayerMapper {
    fn get_conf(&self, layer: LayerId, key: KeyId) -> KeyConf;
}


/// Wrap a hashmap with the LayerMapper interface
pub struct HashMapMapper {
    map: HashMap<(LayerId, KeyId), KeyConf>
}

impl HashMapMapper {
    pub fn new(map: HashMap<(LayerId, KeyId), KeyConf>) -> Self {
        HashMapMapper { map }
    }

    pub fn get_hashmap(&mut self) -> &mut HashMap<(LayerId, KeyId), KeyConf> {
        &mut self.map
    }

    fn get_conf(&self, layer: LayerId, key: KeyId) -> KeyConf {
        self.map.get(&(layer, key)).map(|conf| *conf).unwrap_or(KeyConf::default())
    }
}


/// Simple Mapper implementation to aid testing.
/// Mapper returns `num_keys * layer` + `key`, which yields
/// a deterministic and unique keycode for combination.
/// (So long as the result is not grater than 2^8)
pub struct SimpleMapper {
    num_keys: u8
}

impl SimpleMapper {
    pub fn new(num_keys: u8) -> Self {
        SimpleMapper {num_keys}
    }
}

impl LayerMapper for SimpleMapper {
    fn get_conf(&self, layer: LayerId, key: KeyId) -> KeyConf {
        let key_code = layer * key + self.num_keys;
        let key_action = KeyAction::AddKey(key_code);
        KeyConf::Tap(TapKeyConf{tap: KeyActionSet::Single(key_action)})
    }
}
