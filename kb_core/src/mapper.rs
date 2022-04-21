/// Module introduces types used for layers and a mapper trait
use crate::keys::{LayerId, KeyConf, KeyAction, TapKeyConf, KeyActionSet};
use crate::keyboard::KeyId;
 
use std::collections::HashMap;



/// Trait to ease mapping handling keyboard configurations
/// when multiple layers are supported.
pub trait LayerMapper<T> {
    fn get_conf(&self, layer: LayerId, key: KeyId) -> KeyConf<T>;
}


/// Wrap a hashmap with the LayerMapper interface
pub struct HashMapMapper<T> {
    map: HashMap<(LayerId, KeyId), KeyConf<T>>
}

impl<T: Copy> HashMapMapper<T> {
    pub fn new(map: HashMap<(LayerId, KeyId), KeyConf<T>>) -> Self {
        HashMapMapper { map }
    }

    pub fn get_hashmap(&mut self) -> &mut HashMap<(LayerId, KeyId), KeyConf<T>> {
        &mut self.map
    }

    fn get_conf(&self, layer: LayerId, key: KeyId) -> KeyConf<T> {
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

impl LayerMapper<u8> for SimpleMapper {
    fn get_conf(&self, layer: LayerId, key: KeyId) -> KeyConf<u8> {
        let key_code = layer * key + self.num_keys;
        let key_action = KeyAction::AddKey(key_code);
        KeyConf::Tap(TapKeyConf {tap: KeyActionSet::Single(key_action)})
    }
}