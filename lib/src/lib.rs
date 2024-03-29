pub mod buechi;
pub mod parsing;
extern crate bit_vec;

use crate::buechi::transitions::Transitions;
use bit_vec::BitVec;
use buechi::ltl_to_buechi::ltl_to_büchi;
use buechi::Büchi;
use parsing::LTLFormula;
use std::collections::HashMap;

pub struct KripkeState {
    pub aps: Vec<String>,
    pub id: u64,
    pub start: bool,
}

pub struct KripkeStructure {
    pub states: HashMap<u64, KripkeState>,
    pub transitions: Vec<(u64, u64)>,
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

pub fn kripke_to_büchi(ks: &KripkeStructure, ap_map: &HashMap<String, u8>) -> Büchi<u64> {
    let amount_states = ks.states.len() + 1;
    let mut transitions = Transitions::for_states(amount_states);
    let mut state_infos = Vec::with_capacity(amount_states);
    state_infos.push(u64::MAX);

    let mut state_map = HashMap::<u64, u64>::with_capacity(amount_states);

    for state in ks.states.values() {
        let current_id = state_infos.len() as u64;
        state_map.insert(state.id, current_id);
        state_infos.push(state.id);
        if state.start {
            transitions.add(
                0,
                get_symbol_from_string_aps_with_ap_map(&state.aps, ap_map),
                current_id,
            );
        }
    }

    for (state1, state2) in ks.transitions.iter() {
        transitions.add(
            *state_map.get(state1).unwrap(),
            get_symbol_from_string_aps_with_ap_map(&ks.states.get(state2).unwrap().aps, ap_map),
            *state_map.get(state2).unwrap(),
        );
    }

    Büchi::new(
        state_infos,
        ap_map.len() as u8,
        0,
        transitions,
        BitVec::from_elem(amount_states, true),
    )
}

pub fn ltl_model_check(ks: &KripkeStructure, formula: &str) -> Option<bool> {
    let (ltl, ap_map) = parsing::parse(formula).ok()?;
    let notltl = LTLFormula::Not(Box::new(ltl));

    let model = kripke_to_büchi(ks, &ap_map);

    let generalized_büchi = ltl_to_büchi(&notltl);
    let büchi = Büchi::from_generalized_büchi(generalized_büchi);
    let product = buechi::product::product(&model, &büchi);
    let opt_loop = product.get_loop();
    if opt_loop.is_some() {
        dbg!(&opt_loop);
    }
    Some(opt_loop.is_none())
}
