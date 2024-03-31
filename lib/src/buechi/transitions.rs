use crate::buechi::{State, Symbol};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Transitions {
    transitions: Vec<HashMap<State, Vec<Symbol>>>,
}

impl Transitions {
    pub fn for_states(amount_states: usize) -> Transitions {
        Transitions {
            transitions: vec![HashMap::<State, Vec<Symbol>>::new(); amount_states],
        }
    }

    pub fn add(&mut self, from_state: State, symbol: Symbol, to_state: State) {
        let a = self.transitions.get_mut(from_state as usize).unwrap();
        a.entry(to_state).or_default();
        a.get_mut(&to_state).unwrap().push(symbol);
    }

    pub fn has(&self, from_state: &State, symbol: &Symbol, to_state: &State) -> bool {
        let a = self.transitions.get(*from_state as usize).unwrap();
        let option = a.get(to_state);
        if let Some(state) = option {
            state.contains(symbol)
        } else {
            false
        }
    }

    pub fn get_symbols_from_to(
        &self,
        from_state: &State,
        to_state: &State,
    ) -> impl Iterator<Item = &Symbol> {
        let vec = self
            .transitions
            .get(*from_state as usize)
            .unwrap()
            .get(to_state);
        return if let Some(vec) = vec {
            vec.iter()
        } else {
            [].iter()
        };
    }

    pub fn get_next_states_from_state(
        &self,
        from_state: State,
    ) -> impl Iterator<Item = State> + '_ {
        self.transitions
            .get(from_state as usize)
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
    }

    pub fn get_from_state(&self, from_state: State) -> impl Iterator<Item = (Symbol, State)> + '_ {
        self.transitions
            .get(from_state as usize)
            .unwrap()
            .iter()
            .flat_map(|(state, vec)| vec.iter().map(move |symbol| (*symbol, *state)))
    }

    pub fn get_from_state_with_symbol(
        &self,
        from_state: State,
        with_symbol: Symbol,
    ) -> impl Iterator<Item = State> + '_ {
        self.get_from_state(from_state)
            .filter_map(move |(symbol, to_state)| {
                if symbol == with_symbol {
                    Some(to_state)
                } else {
                    None
                }
            })
    }

    pub fn get_all(&self) -> impl Iterator<Item = (State, Symbol, State)> + '_ {
        return (0..self.transitions.len() as u64).flat_map(move |from_state| {
            self.get_from_state(from_state)
                .map(move |(symbol, to_state)| (from_state, symbol, to_state))
        });
    }
}
