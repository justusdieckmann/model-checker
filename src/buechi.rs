pub mod ltl_to_buechi;

use bit_vec::BitVec;
use std::collections::{HashMap};
use std::ops::{AddAssign, BitOr, BitOrAssign};

type State = u64;
type Symbol = u64;

#[derive(Debug)]
pub struct GeneralizedBüchi<T> {
    state_infos: Vec<T>,
    amount_aps: u8,
    start_state: State,
    transitions: Vec<HashMap<State, Vec<Symbol>>>,
    end_sets: Vec<BitVec>
}

