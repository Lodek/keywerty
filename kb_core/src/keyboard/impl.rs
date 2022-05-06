/// Keyboard trait implementation using state machines

use std::collections::HashMap;
use std::time::Duration;
use std::fmt::Debug;
use std::hash::Hash;

use crate::keyboard::state_machines as sm;
use crate::keyboard::state_machines::KeyStateMachine;
use crate::mapper::LayerMapper;
use super::Keyboard;
use super::Action;
use super::Event;
use crate::keys;


type PendingKeyAction<KeyId, T> = (KeyId, keys::KeyActionSet<T>);


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


pub struct SMKeyboard<KeyId, T, Mapper> {
    default_layer: keys::LayerId,
    layer_mapper: Mapper,
    layer_stack: Vec<keys::LayerId>,
    active_key_actions: HashMap<KeyId, keys::KeyActionSet<T>>,
    state_machines: HashMap<KeyId, Box<dyn KeyStateMachine<KeyId, T>>>,
    settings: SMKeyboardSettings,
}


impl<KeyId, T, Mapper> SMKeyboard<KeyId, T, Mapper> 
where KeyId: Copy + Eq + Hash + Debug + 'static,
      T: Copy + 'static,
      Mapper: LayerMapper<KeyId, T>
{
    pub fn new(default_layer: keys::LayerId, layer_mapper: Mapper, settings: SMKeyboardSettings) -> Self {
        Self {
            settings,
            default_layer,
            layer_mapper: layer_mapper,
            state_machines: HashMap::new(),
            active_key_actions: HashMap::new(),
            layer_stack: Vec::new(),
        }
    }

    fn get_active_layer(&self) -> keys::LayerId {
        self.layer_stack.last().map(|layer| *layer).unwrap_or(self.default_layer) 
    }

    /// receive key id and action, mutate keyboard and possibly generate action
    fn handle_key_action(&mut self, key_action: &keys::KeyAction<T>) -> Option<Action<T>> {
        match key_action {
            keys::KeyAction::SendKey(action) => {
                Some(Action::SendCode(*action))
            },
            keys::KeyAction::StopKey(action) => {
                Some(Action::Stop(*action))
            },
            keys::KeyAction::PushLayer(layer_id) => {
                self.layer_stack.push(*layer_id);
                None
            },
            keys::KeyAction::PopLayer(layer_id) => {
                // FIXME this is incorrect as it will only pop
                // the last layer in the stack.
                self.layer_stack.pop();
                None
            },
            keys::KeyAction::NoOp => {
                None
            },
            keys::KeyAction::ToggleKey(action) => {
                // TODO
                todo!()
            },
            keys::KeyAction::ToggleLayer(action) => {
                // TODO
                todo!()
            },
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
            // debug log
            eprintln!("active state machine for key {:?}", key_id);
        }
        else if let Some(conf) = self.layer_mapper.get_conf(&self.get_active_layer(), key_id) {
            let machine = self.build_machine(key_id, conf);
            self.state_machines.insert(*key_id, machine);
        }
        else {
            // TODO use error log
            eprintln!("Ignored missing key configuration for: layer_id={:?} key_id={:?}", self.get_active_layer(), key_id);
        }
    }

    /// build and initialize the correct state machine from a key conf
    fn build_machine(&mut self, key_id: &KeyId, key_conf: keys::KeyConf<T>) -> Box<dyn KeyStateMachine<KeyId, T>> {
        match key_conf {
            keys::KeyConf::Tap(conf) => {
                let mut ksm = sm::TapKSM::new(*key_id, conf);
                Box::new(ksm)
            }
            keys::KeyConf::Hold(conf) => {
                let mut ksm = sm::HoldKSM::new(self.settings.hold_ksm_delay, *key_id, conf);
                Box::new(ksm)
            },
            keys::KeyConf::DoubleTap(conf) => todo!(),
            keys::KeyConf::DoubleTapHold(conf) => todo!(),
        }
    }

    fn drop_finished_machines(&mut self) {
        let finished_machines = self.state_machines.iter()
            .filter(|(_, machine)| machine.is_finished())
            .map(|(key_id, _)| *key_id)
            .collect::<Vec<_>>();

        for key_id in finished_machines.into_iter() {
            eprintln!("dropped state machine for key: {:?}", key_id);
            self.state_machines.remove(&key_id);
        }
    }
}


impl<KeyId, T, Mapper> Keyboard<KeyId, T> for SMKeyboard<KeyId, T, Mapper>
where KeyId: Hash + Copy + Eq + Debug + 'static,
      T: Copy + 'static + Debug,
      Mapper: LayerMapper<KeyId, T>
{
    fn transition(&mut self, event: Event<KeyId>) -> Vec<Action<T>> {

        eprintln!("handling event: {:?}", event);
        let mut actions = Vec::new();
        let mut pending_action_q = Vec::with_capacity(10);

        if matches!(event, Event::KeyPress(_)) {
            self.handle_key_press_event(&event);
        }

        // map state machine steps into pending key actions
        for (key_id, machine) in self.state_machines.iter_mut() {
            if let Some(key_actions) = machine.transition(&event) {
                eprintln!("transition actions: key_id={:?} actionset={:?}", key_id, key_actions);
                self.active_key_actions.insert(*key_id, key_actions.clone());
                pending_action_q.push((*key_id, key_actions));
            }
        }

        // add cleanup action for finished machines
        for (key_id, machine) in self.state_machines.iter_mut() {
            if machine.is_finished() {
                let actionset = self.active_key_actions.remove(key_id).unwrap();
                let actionset = actionset.invert();
                pending_action_q.push((*key_id, actionset));
            }
        }

        // map pending key actions into actions
        for (key_id, key_actions) in pending_action_q.iter() {
            for key_action in key_actions.get_actions().iter() {
                if let Some(action) = self.handle_key_action(key_action) {
                    actions.push(action);
                }
            }
        }

        eprintln!("active actions : {:?}", self.active_key_actions);
        eprintln!("state machine count: {:?}", self.state_machines.len());
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
