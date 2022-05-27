//! Module introduces types used to abstract keyboard configuration and mapping handling
use std::collections::HashMap;
use std::hash::Hash;

use crate::keys::{KeyAction, KeyActionSet, KeyConf, TapKeyConf};

/// Indetifier for a Layer
pub type LayerId = u8;

/// Trait to abstract keyboard keyconf mapping.
pub trait LayerMapper<KeyId, T> {
    /// Return Keyconf for a layer, key pair.
    fn get_conf(&self, layer: &LayerId, key: &KeyId) -> Option<KeyConf<T>>;
}

/// HashMap implementation for LayerMapper trait
impl<KeyId, T> LayerMapper<KeyId, T> for HashMap<(LayerId, KeyId), KeyConf<T>>
where
    KeyId: Eq + Hash + Copy,
    T: Clone,
{
    fn get_conf(&self, layer: &LayerId, key: &KeyId) -> Option<KeyConf<T>> {
        self.get(&(*layer, *key)).map(|v| v.clone())
    }
}

/// Simple Mapper implementation to aid testing.
/// Mapper returns `key_id * (layer + 1)`.
pub struct SimpleMapper {}

impl LayerMapper<u8, u8> for SimpleMapper {
    fn get_conf(&self, layer: &LayerId, key: &u8) -> Option<KeyConf<u8>> {
        let key_code = (layer + 1) * key;
        let key_action = KeyAction::SendKey(key_code);
        Some(KeyConf::Tap(TapKeyConf {
            tap: KeyActionSet::Single(key_action),
        }))
    }
}

/// LayerMapper which return KeyConf from a HashMap or echoes the input key id
/// as a Tap Key conf.
pub struct MapOrEchoMapper<KeyId>(pub HashMap<(LayerId, KeyId), KeyConf<KeyId>>);

impl<KeyId> LayerMapper<KeyId, KeyId> for MapOrEchoMapper<KeyId>
where
    KeyId: Copy + Eq + Hash,
{
    fn get_conf(&self, layer: &LayerId, key: &KeyId) -> Option<KeyConf<KeyId>> {
        let supplier = |key: KeyId| {
            KeyConf::Tap(TapKeyConf {
                tap: KeyActionSet::Single(KeyAction::SendKey(key)),
            })
        };
        self.0
            .get(&(*layer, *key))
            .map(|v| *v)
            .or(Some(supplier(*key)))
    }
}
