use crate::buechi::transitions::Transitions;
use crate::buechi::Büchi;
use crate::{ModelCheckingError, ModelCheckingErrorKind};
use bit_vec::BitVec;
use std::collections::HashMap;

#[derive(Clone)]
struct KripkeState {
    aps: Vec<String>,
    id: u64,
    start: bool,
}

#[derive(Clone)]
pub struct KripkeBuilder {
    states: HashMap<u64, KripkeState>,
    transitions: Vec<(u64, u64)>,
}

impl Default for KripkeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl KripkeBuilder {
    pub fn new() -> KripkeBuilder {
        KripkeBuilder {
            states: HashMap::new(),
            transitions: vec![],
        }
    }

    pub fn add_state(&mut self, aps: Vec<String>, id: u64, start: bool) {
        self.states.insert(id, KripkeState { id, aps, start });
    }

    pub fn add_transition(&mut self, state_id_1: u64, state_id_2: u64) {
        self.transitions.push((state_id_1, state_id_2));
    }

    fn get_symbol_from_string_aps_with_ap_map(
        string_aps: &Vec<String>,
        ap_map: &HashMap<String, u8>,
    ) -> u64 {
        let mut symbol = 0;
        for string_ap in string_aps {
            if let Some(ap) = ap_map.get(string_ap) {
                symbol |= 1 << ap;
            }
        }
        symbol
    }

    pub fn create_büchi(
        &self,
        ap_map: &HashMap<String, u8>,
    ) -> Result<Büchi<u64>, ModelCheckingError> {
        // TODO Only include dead-state if necessary.
        let amount_states = self.states.len() + 2;
        let mut transitions = Transitions::for_states(amount_states);
        let mut state_infos = Vec::with_capacity(amount_states);
        state_infos.push(u64::MAX);
        state_infos.push(u64::MAX - 1);

        let mut state_map = HashMap::<u64, u64>::with_capacity(amount_states);
        let mut has_start = false;

        for state in self.states.values() {
            let current_id = state_infos.len() as u64;
            state_map.insert(state.id, current_id);
            state_infos.push(state.id);
            if state.start {
                has_start = true;
                transitions.add(
                    0,
                    Self::get_symbol_from_string_aps_with_ap_map(&state.aps, ap_map),
                    current_id,
                );
            }
        }

        if !has_start {
            return Err(ModelCheckingError::new(
                ModelCheckingErrorKind::ModelNoStart,
            ));
        }

        let mut has_successor = BitVec::from_elem(amount_states, false);
        has_successor.set(0, true);
        has_successor.set(1, true);

        for (state1, state2) in self.transitions.iter() {
            let Some(internal_state1) = state_map.get(state1) else {
                return Err(ModelCheckingError::new(
                    ModelCheckingErrorKind::ModelInvalid,
                ));
            };
            let Some(internal_state2) = state_map.get(state2) else {
                return Err(ModelCheckingError::new(
                    ModelCheckingErrorKind::ModelInvalid,
                ));
            };
            let Some(target_state) = self.states.get(state2) else {
                return Err(ModelCheckingError::new(
                    ModelCheckingErrorKind::ModelInvalid,
                ));
            };
            transitions.add(
                *internal_state1,
                Self::get_symbol_from_string_aps_with_ap_map(&target_state.aps, ap_map),
                *internal_state2,
            );
            has_successor.set(*internal_state1 as usize, true);
        }

        if !has_successor.all() {
            // Make structure total.
            has_successor
                .iter()
                .enumerate()
                .filter(|(_, x)| !*x)
                .for_each(|(id, _)| {
                    transitions.add(id as u64, 0, 1);
                });
        }
        transitions.add(1, 0, 1);

        Ok(Büchi::new(
            state_infos,
            ap_map.len() as u8,
            0,
            transitions,
            BitVec::from_elem(amount_states, true),
        ))
    }
}
