pub mod ltl_to_buechi;
mod transitions;

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
    end_sets: Vec<BitVec>
}

#[derive(Debug)]
pub struct Büchi<T> {
    state_infos: Vec<T>,
    amount_aps: u8,
    start_state: State,
    transitions: Transitions,
    end_set: BitVec
}

impl<T> Büchi<T>
where T: Clone {

    pub fn from_generalized_büchi(generalized_büchi: GeneralizedBüchi<T>) -> Büchi<(T, u8)> {
        let amount_endsets = generalized_büchi.end_sets.len();
        if amount_endsets == 0 {
            return Büchi {
                state_infos: generalized_büchi.state_infos.iter().map(|u| { (u.clone(), 0u8) }).collect(),
                amount_aps: generalized_büchi.amount_aps,
                start_state: generalized_büchi.start_state.clone(),
                transitions: generalized_büchi.transitions.clone(),
                end_set: BitVec::from_elem(generalized_büchi.state_infos.len(), true)
            }
        } else if amount_endsets == 1 {
            return Büchi {
                state_infos: generalized_büchi.state_infos.iter().map(|u| { (u.clone(), 0u8) }).collect(),
                amount_aps: generalized_büchi.amount_aps,
                start_state: generalized_büchi.start_state.clone(),
                transitions: generalized_büchi.transitions.clone(),
                end_set: generalized_büchi.end_sets.first().unwrap().clone()
            }
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
                end_set
            }
        }
    }


    pub fn product<T, S>(a1: &Büchi<T>, a2: &Büchi<S>) -> Büchi<(T, S)>{
        if a1.amount_aps != a2.amount_aps {
            panic!("This shouldn't happen.");
        }
        if !a1.end_set.all() && a2.end_set.all() {
            return Self::product(a2, a1);
        }

        let amount_states = (a1.amount_states() * a2.amount_states()) as usize;
        let mut transitions = Transitions::for_states(amount_states);
        let mut state_infos = Vec::<(T, S)>::with_capacity(amount_states);
        let mut end_set = BitVec::with_capacity(amount_states);
        if a1.end_set.all() {
            for state1 in a1.state_infos {
                for state2 in a2.state_infos {
                    state_infos.push((*state1, *state2));
                    end_set.push(a2.end_set.get(state2).unwrap())
                }
            }
            for (from_state1, symbol, to_state1) in a1.transitions.get_all() {
                for from_state2 in 0u64 .. a2.amount_states() {
                    for to_state2 in a2.transitions.get_from_state_with_symbol(from_state2, symbol) {
                        transitions.add(from_state1 * a2.amount_states() + from_state2, symbol,
                                        to_state1 * a2.amount_states() + to_state2);
                    }
                }
            }
        } else {
            todo!("Only simple product automata implemented until now.")
        }


        return Büchi {
            state_infos,
            amount_aps,
            start_state: a1.start_state * a2.amount_states() + a2.start_state,
            transitions,
            end_set
        }
    }

    pub fn amount_states(&self) -> u64 {
        return self.state_infos.len() as u64;
    }

}

