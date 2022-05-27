mod eager_hold_ksm;
mod helpers;
mod hold_ksm;
/// Keyboard trait implementation using state machines
///
/// Some key activation modes are stateful in nature and depends
/// on the state of the other keys in the keyboard to perform an action.
/// The KeyboardStateM is a trait that can be implemented for stateful
/// activation modes.
///
/// Each time a stateful key is pressed, a new state machine should be created
/// to handle that state.
mod tap_ksm;
//mod double_tap_ksm;
//mod double_tap_hold_ksm;

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Duration;

use super::Action;
use super::Event;
use super::Keyboard;
use crate::keys;
use crate::keys::KeyActionSet;
use crate::mapper::LayerMapper;
use eager_hold_ksm::EagerHoldKSM;
use hold_ksm::HoldKSM;
use tap_ksm::TapKSM;
//use double_tap_ksm::DoubleTapKSM;
//use double_tap_hold_ksm::DoubleTapHoldKSM;

use log;

/// SMKeyboard is a state machiene orchestrator, more specifically it coordinates
/// KeyStateMachine types.
///
/// KeyStateMachine (KSM) models a - potentially stateful - key and its transitions.
///
/// KSM is state machine like, but does not comfort to the standard state machine
/// notions in that it produces values.
/// KSM receives `Event` objects and return `Option<KeyActionSet>` when transitioned.
/// The idea is that KSM encapsulates the states and yields actionable values
/// from which `SMKeyboard` should act.
/// Actually implementing KSM as a state machine is left up to each concrete type,
/// however the state machine proved to be a good model for keys.
///
/// Some details about the interface:
/// - KSM handles the activation of *one* key, the "watched key".
/// - KSM receives inputs and optionally yields an action.
/// - KSMs will reach a "finished" state, after which it will be discarted.
///
/// Once a KSM is "finished", SMKeyboard will drop it.
/// The semantics of "finished" depend on the behavior being modeled, but can be understood as
/// "A key that has reached its final state and will no longer produce a result".
/// Prior to being dropped however, KSMs are queried for cleanup actions that the main
/// keyboard should perform.
/// The cleanup logic in most cases (but not always) will undo the actions
/// performed by the state machine.
///
/// It's up to each machine to properly undo the actions it perform, however
/// it must be noted that since machines mutate the state of the main keyboard
/// attention is required to not generate unexpected behaviors.
pub trait KeyStateMachine<KeyId, T> {
    /// Steps the state machine from the current events
    /// Each step may return a KeyActionSet.
    fn transition<'a>(&mut self, event: &Event<KeyId>) -> Option<KeyActionSet<T>>;

    /// Return the key for which the KSM is reponsible.
    fn get_watched_key(&self) -> &KeyId;

    /// Check whether the machine's current state is one of its accepting states.
    /// A finished state machine will be dropped after performing the cleanup actions.
    fn is_finished(&self) -> bool;

    /// Fetch actions that should performed to cleanup the state machine.
    /// Cleanup is done after a machine is finished and before it is dropped.
    fn get_cleanup_actions(&self) -> &[KeyActionSet<T>];
}

#[derive(Debug, Clone, Copy)]
pub struct SMKeyboardSettings {
    pub hold_ksm_delay: Duration,

    pub dtksm_retap_delay: Duration,
    pub dtksm_hold_delay: Duration,

    pub dthksm_retap_delay: Duration,
    pub dthksm_hold_delay: Duration,
}

impl Default for SMKeyboardSettings {
    fn default() -> Self {
        SMKeyboardSettings {
            hold_ksm_delay: Duration::from_millis(750),

            dtksm_retap_delay: Duration::from_millis(100),
            dtksm_hold_delay: Duration::from_millis(100),

            dthksm_retap_delay: Duration::from_millis(100),
            dthksm_hold_delay: Duration::from_millis(100),
        }
    }
}

/// `SMKeyboard` implements the `Keyboard` trait defined in this crate.
/// SMKeyboard implements its logic through a special data type called `KeyStateMachine` (KSM).
/// KSMs are responsible for modeling the states a key transition through during its lifetime.
/// SMKeyboard can be seen as an orchestrator for KSMs.
///
/// On a KeyPress Event, SMKb verified whether there exists a State Machine for that key,
/// if there is not it attempts to fetch the maching key's KeyConf and creates a KSM for it.
/// The key handling logic is then delegated to the KSM.
///
/// SMKb notifies every active state machine of the received event.
/// Each machine may generate an action which shall be handled by SMKb.
///
/// Once a KSM is finished, SMKb will perform any cleanup actions and proceed to drop it.
pub struct SMKeyboard<KeyId, T, Mapper> {
    default_layer: keys::LayerId,
    layer_mapper: Mapper,
    layer_stack: Vec<keys::LayerId>,
    state_machines: HashMap<KeyId, Box<dyn KeyStateMachine<KeyId, T>>>,
    state_machine_order: Vec<KeyId>,
    settings: SMKeyboardSettings,
}

impl<KeyId, T, Mapper> SMKeyboard<KeyId, T, Mapper>
where
    KeyId: Copy + Eq + Hash + Debug + 'static,
    T: Clone + 'static,
    Mapper: LayerMapper<KeyId, T>,
{
    pub fn new(
        default_layer: keys::LayerId,
        layer_mapper: Mapper,
        settings: SMKeyboardSettings,
    ) -> Self {
        Self {
            settings,
            default_layer,
            layer_mapper: layer_mapper,
            state_machines: HashMap::new(),
            layer_stack: Vec::new(),
            state_machine_order: Vec::new(),
        }
    }

    fn get_active_layer(&self) -> keys::LayerId {
        self.layer_stack
            .last()
            .map(|layer| *layer)
            .unwrap_or(self.default_layer)
    }

    /// receive key id and action, mutate keyboard and possibly generate action
    fn handle_key_action(&mut self, key_action: &keys::KeyAction<T>) -> Option<Action<T>> {
        match key_action {
            keys::KeyAction::SendKey(data) => Some(Action::SendCode(data.clone())),
            keys::KeyAction::StopKey(data) => Some(Action::Stop(data.clone())),
            keys::KeyAction::PushLayer(layer_id) => {
                self.layer_stack.push(*layer_id);
                None
            }
            keys::KeyAction::PopLayer(_) => {
                // FIXME this is incorrect as it will only pop
                // the last layer in the stack.
                self.layer_stack.pop();
                None
            }
            keys::KeyAction::NoOp => None,
        }
    }

    /// Handle key press by verifying whether there exists a state machine to process the pressed key.
    /// Create a state machine and initialize it if necessary.
    fn handle_key_press_event(&mut self, event: &Event<KeyId>) {
        if !event.is_key_press() {
            return;
        }

        let key_id = event.get_key_id().unwrap();

        // A pressed key for which there exists an active state machine
        // does not need handling, as the machine will be subsequently
        // executed in the transition phase.
        if self.state_machines.contains_key(key_id) {
            log::debug!("active state machine for key {:?}", key_id);
        } else if let Some(conf) = self.layer_mapper.get_conf(&self.get_active_layer(), key_id) {
            let machine = self.build_machine(key_id, conf);
            self.state_machines.insert(*key_id, machine);
            self.state_machine_order.push(*key_id);
        } else {
            log::error!(
                "Ignored missing key configuration for: layer_id={:?} key_id={:?}",
                self.get_active_layer(),
                key_id
            );
        }
    }

    /// build and initialize the correct state machine from a key conf
    fn build_machine(
        &mut self,
        key_id: &KeyId,
        key_conf: keys::KeyConf<T>,
    ) -> Box<dyn KeyStateMachine<KeyId, T>> {
        match key_conf {
            keys::KeyConf::Tap(conf) => {
                let ksm = TapKSM::new(*key_id, conf);
                Box::new(ksm)
            }
            keys::KeyConf::Hold(conf) => {
                let ksm = HoldKSM::new(self.settings.hold_ksm_delay, *key_id, conf);
                Box::new(ksm)
            }
            keys::KeyConf::EagerHold(conf) => {
                let ksm = EagerHoldKSM::new(self.settings.hold_ksm_delay, *key_id, conf);
                Box::new(ksm)
            }
            keys::KeyConf::DoubleTap(_) => todo!(),
            keys::KeyConf::DoubleTapHold(_) => todo!(),
        }
    }

    fn drop_finished_machines(&mut self) {
        let finished_machines = self
            .state_machines
            .iter()
            .filter(|(_, machine)| machine.is_finished())
            .map(|(key_id, _)| *key_id)
            .collect::<Vec<_>>();

        let is_not_finished = |key_id: &KeyId| !finished_machines.contains(key_id);

        self.state_machine_order.retain(is_not_finished);

        for key_id in finished_machines.into_iter() {
            log::debug!("dropped state machine for key: {:?}", key_id);
            self.state_machines.remove(&key_id);
        }
    }
}

impl<KeyId, T, Mapper> Keyboard<KeyId, T> for SMKeyboard<KeyId, T, Mapper>
where
    KeyId: Hash + Copy + Eq + Debug + 'static,
    T: Clone + 'static + Debug,
    Mapper: LayerMapper<KeyId, T>,
{
    fn transition(&mut self, event: Event<KeyId>) -> Vec<Action<T>> {
        log::debug!("handling event: {:?}", event);
        let mut actions = Vec::new();
        let mut pending_action_q = Vec::with_capacity(10);

        if matches!(event, Event::KeyPress(_)) {
            self.handle_key_press_event(&event);
        }

        // map state machine steps into pending key actions
        for key_id in self.state_machine_order.iter() {
            let machine = self.state_machines.get_mut(key_id).unwrap();
            if let Some(key_actions) = machine.transition(&event) {
                log::debug!(
                    "transition actions: key_id={:?} actionset={:?}",
                    key_id,
                    key_actions
                );
                pending_action_q.push((*key_id, key_actions));
            }
        }

        // add cleanup action for finished machines
        for (key_id, machine) in self.state_machines.iter_mut() {
            if machine.is_finished() {
                for actionset in machine.get_cleanup_actions() {
                    pending_action_q.push((*key_id, actionset.clone()));
                }
            }
        }

        // map pending key actions into actions
        for (_, key_actions) in pending_action_q.iter() {
            for key_action in key_actions.get_actions().iter() {
                if let Some(action) = self.handle_key_action(key_action) {
                    actions.push(action);
                }
            }
        }

        log::debug!("state machine count: {:?}", self.state_machines.len());
        self.drop_finished_machines();

        actions
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::mapper::SimpleMapper;

    /*
    #[test]
    fn test_sanity_press_then_release_with_simple_mapper() {
        let num_keys = 10;
        let simple_mapper = SimpleMapper::new(num_keys);
        let settings = SMKeyboardSettings::default();
        let mut keyboard = SMKeyboard::new(num_keys, 0, simple_mapper, settings);

        let press_actions = keyboard.transition(Event::KeyPress(1));
        let release_actions = keyboard.transition(Event::KeyRelease(1));

        assert_eq!(press_actions[0], Action::SendCode(1));
        assert_eq!(release_actions[0], Action::Stop(1));
    }
    */

    // test layer setting
    // test state machine
}
