use std::time::{Duration};
use std::collections::BTreeMap;

use keyboard_interface::{Keyboard, Action, Event, KeyId};
use keyboard_interface::map::{LayerMapper, KeyConf, KeyAction, KeyActionSet, TapKeyConf, HoldKeyConf, DoubleTapKeyConf, DoubleTapHoldKeyConf, LayerId};
use crate::statem::{KeyStateMachine, KSMInit, HoldKSM, DoubleTapKSM, DoubleTapHoldKSM};

pub struct SMKeyboard<LayerMapperImpl> {
    num_keys: u8,
    default_layer: LayerId,
    layer_mapper: LayerMapperImpl,
    stateful_handling: Option<Box<dyn KeyStateMachine>>,
    layer_stack: Vec<LayerId>,
    key_actions_map: BTreeMap<KeyId, KeyActionSet>
}


impl<LayerMapperImpl> SMKeyboard<LayerMapperImpl> 
where LayerMapperImpl: LayerMapper
{
    pub fn new(num_keys: u8, default_layer: LayerId, layer_mapper: LayerMapperImpl) -> Self {
        Self {
            num_keys,
            default_layer,
            layer_mapper,
            stateful_handling: None,
            layer_stack: Vec::with_capacity(num_keys.into()),
            key_actions_map: BTreeMap::new(),
        }
    }

    fn get_layer(&self) -> LayerId {
        self.layer_stack.last().map(|layer| layer.clone()).unwrap_or(self.default_layer) 
    }


    fn handle_state_machine<'a>(&mut self, action_queue: &'a mut Vec<Action>, event: Event) {
        let mut machine = self.stateful_handling.take().unwrap();
        let watched_key = machine.get_watched_key();
        
        if let Some(actionset) = machine.transition(event) {
            self.apply_actionset(machine.get_watched_key(), actionset, action_queue);
            self.stateful_handling = None;
        }
        else {
            self.stateful_handling = Some(machine);
        }

        // huh?
        if watched_key != event.get_key_id() && !event.is_key_press() {
            self.handle_event(action_queue, event);
        }
    }

    fn handle_event(&mut self, action_queue: &mut Vec<Action>, event: Event) {
        match event {
            Event::KeyPress(key) => {
                self.handle_key_press(key, action_queue);
            },
            Event::KeyRelease(key_id) => {
                self.handle_key_release(action_queue, key_id);
            }
            Event::Poll => { },
        }
    }


    fn handle_key_press(&mut self, key: KeyId, action_queue: &mut Vec<Action>) {
        match self.layer_mapper.get_conf(self.get_layer(), key) {
            KeyConf::Tap(tap_conf) => {
                let actionset = tap_conf.0;
                self.apply_actionset(key, actionset, action_queue);
                self.key_actions_map.insert(key, actionset);
            },
            // FIXME so much repetition dude
            KeyConf::Hold(key_conf) => {
                // TODO where should the duration for the machines come from?
                let mut ksm = HoldKSM::new(Duration::from_millis(100));
                ksm.init_machine(key, key_conf);
                self.stateful_handling = Some(Box::new(ksm));
            },
            KeyConf::DoubleTap(key_conf) => {
                // TODO where should the duration for the machines come from?
                let mut ksm = DoubleTapKSM::new(Duration::from_millis(100), Duration::from_millis(100));
                ksm.init_machine(key, key_conf);
                self.stateful_handling = Some(Box::new(ksm));
            },
            KeyConf::DoubleTapHold(key_conf) => {
                // TODO where should the duration for the machines come from?
                let mut ksm = DoubleTapHoldKSM::new(Duration::from_millis(100), Duration::from_millis(100));
                ksm.init_machine(key, key_conf);
                self.stateful_handling = Some(Box::new(ksm));
            }
        }
    }

    fn handle_key_release(&mut self, action_queue: &mut Vec<Action>, key_id: KeyId) {
        // In case the user "releases" a key that wasn't pressed before,
        // a Default NoOp action will happen
        let actionset =  self.key_actions_map.remove(&key_id).unwrap_or_default();
        self.undo_actionset(key_id, actionset, action_queue);
    }

    fn apply_keyaction(&mut self, key: KeyId, action: KeyAction) -> Option<Action> {
        match action {
            KeyAction::AddKey(key_code) => Some(Action::SendCode(key_code)),
            KeyAction::SetLayer(layer_id) => {
                self.layer_stack.push(layer_id);
                None
            },
            KeyAction::NoOp => None
        }
    }

    fn undo_keyaction(&mut self, key: KeyId, action: KeyAction) -> Option<Action> {
        match action {
            KeyAction::AddKey(key_code) => Some(Action::Stop(key_code)),

            // FIXME this is wrong. *Any* released layer key will pop the last 
            // inserted layer, which is incorrect.
            // If key 1 pushes layer A, key 2 pushes layer B and key 1 gets released
            // while key 2 is pressed, layer B will be popped and it will go to layer A.
            // 
            // A better impl of this would be an ordered list of layer pushes
            // and SetLayer release would remove the first entry that matches
            // the KeyId, LayerId pair.
            KeyAction::SetLayer(layer_id) => {
                self.layer_stack.pop();
                None
            },
            KeyAction::NoOp => None
        }
    }

    // TODO inline this whole function
    fn actionset_apply<F>(&mut self, key: KeyId, actionset: KeyActionSet, queue: &mut Vec<Action>, mut supplier: F) 
    where F: FnMut(&mut Self, KeyId, KeyAction) -> Option<Action> 
    {
        // TODO should convert this to a macro or an inline function so there's no overhead
        let mut append_if_some = |opt: Option<Action>| if opt.is_some() {queue.push(opt.unwrap())};

        // FIXME generate macro to clean repetition?
        match actionset {
            KeyActionSet::Single(ka1) => {
                let opt = supplier(self, key, ka1);
                append_if_some(opt);
            }
            KeyActionSet::Double(ka1, ka2) => {
                let opt = supplier(self, key, ka1);
                append_if_some(opt);

                let opt = supplier(self, key, ka2);
                append_if_some(opt);
            },
            KeyActionSet::Triple(ka1, ka2, ka3) => {
                let opt = supplier(self, key, ka1);
                append_if_some(opt);

                let opt = supplier(self, key, ka2);
                append_if_some(opt);
                
                let opt = supplier(self, key, ka3);
                append_if_some(opt);
            }
        }
    }

    fn apply_actionset(&mut self, key: KeyId, actionset: KeyActionSet, action_queue: &mut Vec<Action>) {
        self.actionset_apply(key, actionset, action_queue, Self::apply_keyaction)
    }

    fn undo_actionset(&mut self, key: KeyId, actionset: KeyActionSet, action_queue: &mut Vec<Action>) {
        self.actionset_apply(key, actionset, action_queue, Self::undo_keyaction)
    }
}


impl<LayerMapperImpl> Keyboard for SMKeyboard<LayerMapperImpl>
where LayerMapperImpl: LayerMapper
{
    fn transition(&mut self, event: Event) -> Vec<Action> {
        let mut actions = Vec::with_capacity(5); // magic number ew

        if self.stateful_handling.is_some() {
            self.handle_state_machine(&mut actions, event);
        }
        else {
            self.handle_event(&mut actions, event);
        }
        return actions;
    }
}

// Does the state machine paradigm works for key releases? I guess it's possible to implement a
// statemachine that will cause a faulty behavior because the algorithm assumes that once a stateM
// returns something, it's done and it can be discarded.
// Suppose the state machine decides to perform an action once a key is released.
// Then once the action is performed, in theory, the action will never be undone because
// next time the key is pressed a new state machine will be created
// i guess that's one scenarion in which the state machine can lead to a weird state
//
// TODO another detail i need to take care of is how the state machine will interact when two stateful
// keys are pressed.
// say key 1 and 2 are stateful
// what happens if i'm handling the stateM for key 1 and then key 2 is pressed?
