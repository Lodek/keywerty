//! Helper functions for state machine keyboard
use super::KeyStateMachine;
use crate::keyboard::Event;

/// checks whether the key from the current event is a key press for the watched key
pub fn is_watched_key_pressed<KSM, KeyId, T>(ksm: &KSM, event: &Event<KeyId>) -> bool
where
    KSM: KeyStateMachine<KeyId, T>,
    KeyId: PartialEq,
{
    matches!(event, Event::KeyPress(key_id) if key_id == ksm.get_watched_key())
}
