/// Module introduces types used for layers and a mapper trait
use crate::keyboard::{KeyCode, KeyId};

use std::collections::HashMap;


pub type LayerId = u16;


/// Trait to ease mapping handling keyboard configurations
/// when multiple layers are supported.
pub trait LayerMapper<K> {
    fn get_conf(&self, layer: LayerId, key: KeyId) -> K;
}


/// Wrap a hashmap with the LayerMapper interface
pub struct HashMapMapper<T> {
    map: HashMap<(LayerId, KeyId), T>
}

impl<T> HashMapMapper<T> {
    pub fn new(map: HashMap<(LayerId, KeyId), T>) -> Self {
        HashMapMapper { map }
    }

    pub fn get_hashmap(&mut self) -> &mut HashMap<(LayerId, KeyId), KeyConf> {
        &mut self.map
    }
}

impl<T: Default> LayerMapper<T> for HashMapMapper<T> {
    fn get_conf(&self, layer: LayerId, key: KeyId) -> T {
        self.map.get((layer, key)).map(|conf| conf).unwrap_or(T::default())
    }
}





// FIXME does this still make sense?

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
    fn get_conf(&self, layer: LayerId, key: KeyId) -> KeyConf {
        let key_code = layer * self.num_keys + key;
        let key_action = KeyAction::AddKey(key_code);
        KeyConf::Tap(TapKeyConf(KeyActionSet::Single(key_action)))
    }
}
