use bit_vec::BitVec;
use crate::buechi::{Büchi};
use crate::buechi::transitions::Transitions;

pub fn product<T, S>(a1: &Büchi<T>, a2: &Büchi<S>) -> Büchi<(T, S)> where T: Clone, S: Clone {
    if a1.amount_aps != a2.amount_aps {
        panic!("This shouldn't happen.");
    }
    if !a1.end_set.all() && a2.end_set.all() {
        let büchi = product::<S, T>(a2, a1);
        let state_infos: Vec<(T, S)> = büchi.state_infos.iter().map(|(a, b)| {
            return (b.clone(), a.clone())
        }).collect();
        return Büchi {
            state_infos,
            amount_aps: büchi.amount_aps,
            start_state: büchi.start_state,
            transitions: büchi.transitions,
            end_set: büchi.end_set
        }
    }

    let amount_states = (a1.amount_states() * a2.amount_states()) as usize;
    let mut transitions = Transitions::for_states(amount_states);
    let mut state_infos = Vec::<(T, S)>::with_capacity(amount_states);
    let mut end_set = BitVec::with_capacity(amount_states);
    if a1.end_set.all() {
        for state1 in &a1.state_infos {
            for i in 0 .. a2.state_infos.len() {
                state_infos.push((state1.clone(), a2.state_infos.get(i).unwrap().clone()));
                end_set.push(a2.end_set.get(i).unwrap())
            }
        }
        for (from_state1, symbol, to_state1) in a1.transitions.get_all() {
            for from_state2 in 0u64..a2.amount_states() {
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
        amount_aps: a1.amount_aps,
        start_state: a1.start_state * a2.amount_states() + a2.start_state,
        transitions,
        end_set,
    };
}