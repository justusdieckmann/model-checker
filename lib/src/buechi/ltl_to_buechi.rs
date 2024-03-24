use std::collections::{HashMap, HashSet};
use std::ops::{AddAssign, BitOr, BitOrAssign};
use bit_vec::BitVec;
use crate::buechi::{GeneralizedBüchi, State, Transitions};
use crate::parsing::LTLFormula;

fn get_aps_in_ltl(ltl: &LTLFormula, aps: &mut HashSet<u8>) {
    match ltl {
        LTLFormula::AP(id) => { aps.insert(*id); },
        LTLFormula::Not(phi) => { get_aps_in_ltl(phi, aps); }
        LTLFormula::And(phi1, phi2) => { get_aps_in_ltl(phi1, aps); get_aps_in_ltl(phi2, aps); }
        LTLFormula::Next(phi) => { get_aps_in_ltl(phi, aps); }
        LTLFormula::Until(phi1, phi2) => { get_aps_in_ltl(phi1, aps); get_aps_in_ltl(phi2, aps); }
    }
}

pub fn ltl_to_büchi(ltl: &LTLFormula) -> GeneralizedBüchi<u64> {
    let mut aps_set = HashSet::<u8>::new();
    get_aps_in_ltl(ltl, &mut aps_set);
    let ap_count = aps_set.len() as u8;
    let ap_bitmask = (1u64 << ap_count) - 1;

    let mut states = vec![0];

    let mut constraints = Vec::<CompareTheThing>::new();
    let mut end_set_functions = Vec::<GetValueForThing>::new();

    let complete_function = ltl_to_büchi_recursive(
        ltl, &mut states, &mut constraints, &mut end_set_functions, &mut HashMap::<&LTLFormula, u8>::new(), &mut ap_count.clone()
    );

    let amount_states = states.len() + 1;
    let start_id = states.len() as u64;
    let mut transitions = Transitions::for_states(amount_states);
    let mut end_sets = vec![BitVec::with_capacity(states.len() + 1); end_set_functions.len()];

    for (i, state) in states.iter().enumerate() {
        for (i2, state2) in states.iter().enumerate() {
            if constraints.iter().all(|constraint| {
                constraint.compare(state, state2)
            }) {
                transitions.add(i as u64, *state2 & ap_bitmask, i2 as u64);
            }
        }

        if complete_function.get(state) {
            transitions.add(start_id, *state & ap_bitmask, i as u64);
        }

        for (i2, end_set_function) in end_set_functions.iter().enumerate() {
            end_sets.get_mut(i2).unwrap().push(end_set_function.get(state));
        }
    }

    states.push(u64::MAX);
    for (i2, _) in end_set_functions.iter().enumerate() {
        end_sets.get_mut(i2).unwrap().push(false);
    }

    return GeneralizedBüchi {
        state_infos: states,
        amount_aps: ap_count,
        start_state: start_id,
        transitions,
        end_sets
    };
}

#[derive(Clone)]
enum GetValueForThing {
    And(Box<GetValueForThing>, Box<GetValueForThing>),
    Not(Box<GetValueForThing>),
    Lookup(u8)
}

impl GetValueForThing {
    fn get(&self, s: &State) -> bool {
        return match self {
            GetValueForThing::And(a1, a2) => a1.get(s) && a2.get(s),
            GetValueForThing::Not(a) => !a.get(s),
            GetValueForThing::Lookup(a) => (s & (1 << a)) != 0,
        }
    }
}

enum CompareTheThing {
    Next(GetValueForThing, GetValueForThing),
    Until(GetValueForThing, GetValueForThing, GetValueForThing)
}

impl CompareTheThing {
    fn compare(&self, q1: &State, q2: &State) -> bool {
        return match self {
            CompareTheThing::Next(xphi, phi) => xphi.get(q1) == phi.get(q2),
            CompareTheThing::Until(phi1, phi2, phi1_u_phi2) => {
                let q1_has_u = phi1_u_phi2.get(q1);
                q1_has_u && phi2.get(q1) || !q1_has_u && !phi1.get(q1) || q1_has_u == phi1_u_phi2.get(q2)
            }
        }
    }
}

fn ltl_to_büchi_recursive<'a>(
    ltl: &'a LTLFormula,
    states: &mut Vec<State>,
    constraints: &mut Vec<CompareTheThing>,
    end_set_functions: &mut Vec<GetValueForThing>,
    bit_usage: &mut HashMap<&'a LTLFormula, u8>,
    bits_used: &mut u8
) -> GetValueForThing {

    if bit_usage.contains_key(ltl) {
        let id = bit_usage.get(ltl).unwrap();
        return GetValueForThing::Lookup(*id);
    }

    return match ltl {
        LTLFormula::Not(phi) => { GetValueForThing::Not(Box::new(ltl_to_büchi_recursive(phi, states, constraints, end_set_functions, bit_usage, bits_used))) }
        LTLFormula::And(phi1, phi2) => { GetValueForThing::And(Box::new(ltl_to_büchi_recursive(phi1, states, constraints, end_set_functions, bit_usage, bits_used)), Box::new(ltl_to_büchi_recursive(phi2, states, constraints, end_set_functions, bit_usage, bits_used))) }
        LTLFormula::AP(id) => {
            let mut tempvec = Vec::<State>::new();
            for state in &mut *states {
                tempvec.push(state.bitor(1 << id));
            }
            states.append(&mut tempvec);
            bit_usage.insert(ltl, *id);
            GetValueForThing::Lookup(*id)
        }
        LTLFormula::Next(phi) => {
            let val = ltl_to_büchi_recursive(phi, states, constraints, end_set_functions, bit_usage, bits_used);

            let id = *bits_used;
            bits_used.add_assign(1);
            bit_usage.insert(ltl, id);


            let mut tempvec = Vec::<State>::new();
            for state in &mut *states {
                tempvec.push(state.bitor(1 << id));
            }
            states.append(&mut tempvec);
            constraints.push(CompareTheThing::Next(GetValueForThing::Lookup(id), val));
            GetValueForThing::Lookup(id)
        }
        LTLFormula::Until(phi1, phi2) => {
            let val1 = ltl_to_büchi_recursive(phi1, states, constraints, end_set_functions, bit_usage, bits_used);
            let val2 = ltl_to_büchi_recursive(phi2, states, constraints, end_set_functions, bit_usage, bits_used);
            let id = *bits_used;
            bits_used.add_assign(1);

            bit_usage.insert(ltl, id);
            let mut tempvec = Vec::<State>::new();
            for state in &mut *states {
                match (val1.get(state), val2.get(state)) {
                    (_, true) => { state.bitor_assign(1 << id); },
                    (true, false) => { tempvec.push(state.bitor(1 << id)); },
                    (false, false) => {}
                }
            }
            states.append(&mut tempvec);
            constraints.push(CompareTheThing::Until(val1.clone(), val2.clone(), GetValueForThing::Lookup(id)));
            end_set_functions.push(GetValueForThing::Not(
                Box::new(GetValueForThing::And(
                    Box::new(GetValueForThing::Lookup(id)),
                    Box::new(GetValueForThing::Not(Box::new(val2)))
                ))
            ));
            GetValueForThing::Lookup(id)
        }
    }
}
