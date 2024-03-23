pub mod ltl_to_buechi;
pub(crate) mod transitions;
pub mod product;

use bit_vec::BitVec;
use crate::buechi::transitions::Transitions;

type State = u64;
type Symbol = u64;

#[derive(Debug)]
pub struct GeneralizedBüchi<T> {
    state_infos: Vec<T>,
    amount_aps: u8,
    start_state: State,
    transitions: Transitions,
    end_sets: Vec<BitVec>,
}

#[derive(Debug)]
pub struct Büchi<T> {
    state_infos: Vec<T>,
    amount_aps: u8,
    start_state: State,
    transitions: Transitions,
    end_set: BitVec,
}

struct EmptinessStruct {
    stack: Vec<Symbol>,
    inner: BitVec,
    outer_begun: BitVec,
    outer_finished: BitVec,
}

impl<T> Büchi<T>
    where T: Clone {

    pub fn new(state_infos: Vec<T>, amount_aps: u8, start_state: State, transitions: Transitions, end_set: BitVec) -> Self {
        Self { state_infos, amount_aps, start_state, transitions, end_set }
    }

    pub fn from_generalized_büchi(generalized_büchi: GeneralizedBüchi<T>) -> Büchi<(T, u8)> {
        let amount_endsets = generalized_büchi.end_sets.len();
        if amount_endsets == 0 {
            return Büchi {
                state_infos: generalized_büchi.state_infos.iter().map(|u| { (u.clone(), 0u8) }).collect(),
                amount_aps: generalized_büchi.amount_aps,
                start_state: generalized_büchi.start_state.clone(),
                transitions: generalized_büchi.transitions.clone(),
                end_set: BitVec::from_elem(generalized_büchi.state_infos.len(), true),
            };
        } else if amount_endsets == 1 {
            return Büchi {
                state_infos: generalized_büchi.state_infos.iter().map(|u| { (u.clone(), 0u8) }).collect(),
                amount_aps: generalized_büchi.amount_aps,
                start_state: generalized_büchi.start_state.clone(),
                transitions: generalized_büchi.transitions.clone(),
                end_set: generalized_büchi.end_sets.first().unwrap().clone(),
            };
        } else {
            let mut infos = Vec::<(T, u8)>::with_capacity(generalized_büchi.state_infos.len() * amount_endsets);
            let mut transitions = Transitions::for_states(infos.len());
            for i in 0..amount_endsets {
                for state_info in generalized_büchi.state_infos.iter() {
                    infos.push((state_info.clone(), i as u8));
                }
                for (state1, symbol, state2) in generalized_büchi.transitions.get_all() {
                    let target_plane = if generalized_büchi.end_sets.get(i).unwrap().get(state1 as usize).unwrap() {
                        (i + 1) % amount_endsets
                    } else {
                        i
                    };
                    transitions.add((i * generalized_büchi.state_infos.len()) as u64 + state1, symbol,
                                    (target_plane * generalized_büchi.state_infos.len()) as u64 + state2)
                }
            }
            let mut end_set = BitVec::from_elem(generalized_büchi.end_sets.len() * (amount_endsets - 1), false);
            end_set.reserve_exact(generalized_büchi.end_sets.len());
            for b in generalized_büchi.end_sets.last().unwrap() {
                end_set.push(b);
            }
            return Büchi {
                state_infos: infos,
                amount_aps: generalized_büchi.amount_aps,
                start_state: generalized_büchi.start_state,
                transitions,
                end_set,
            };
        }
    }

    pub fn amount_states(&self) -> u64 {
        return self.state_infos.len() as u64;
    }

    fn dfs_cycle(&self, s: &mut EmptinessStruct, q: State) -> bool {
        s.inner.set(q as usize, true);
        for qnext in self.transitions.get_next_states_from_state(q) {
            if s.outer_finished.get(qnext as usize).unwrap() {
                s.stack.push(qnext);
                return true;
            } else if !s.inner.get(qnext as usize).unwrap() {
                self.dfs_cycle(s, qnext);
                return true;
            }
        }
        return false;
    }

    fn dfs(&self, s: &mut EmptinessStruct, q: State) -> bool {
        s.outer_begun.set(q as usize, true);
        for qnext in self.transitions.get_next_states_from_state(q) {
            if !s.outer_begun.get(qnext as usize).unwrap() {
                if self.dfs(s, qnext) {
                    s.stack.push(q);
                    return true;
                }
            }
        }
        if self.end_set.get(q as usize).unwrap() && self.dfs_cycle(s, q) {
            s.stack.push(q);
        }
        s.outer_finished.set(q as usize, false);
        return false;
    }

    pub fn is_empty(&self) -> bool {
        let mut state = EmptinessStruct {
            stack: Vec::<Symbol>::new(),
            inner: BitVec::from_elem(self.amount_states() as usize, false),
            outer_begun: BitVec::from_elem(self.amount_states() as usize, false),
            outer_finished: BitVec::from_elem(self.amount_states() as usize, false),
        };

        return !self.dfs(&mut state, self.start_state)
    }
}

