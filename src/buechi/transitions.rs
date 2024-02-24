use std::collections::HashMap;
use crate::buechi::{State, Symbol};

#[derive(Debug, Clone)]
pub struct Transitions {
    transitions: Vec<HashMap<State, Vec<Symbol>>>
}

impl Transitions {

    pub fn for_states(amount_states: usize) -> Transitions {
        return Transitions {
            transitions: vec![HashMap::<State, Vec<Symbol>>::new(); amount_states]
        }
    }

    pub fn add(&mut self, from_state: State, symbol: Symbol, to_state: State) {
        let a = self.transitions.get_mut(from_state as usize).unwrap();
        if !a.contains_key(&to_state) {
            a.insert(to_state, Vec::<Symbol>::new());
        }
        a.get_mut(&to_state).unwrap().push(symbol);
    }

    pub fn has(&self, from_state: &State, symbol: &Symbol, to_state: &State) -> bool {
        let a = self.transitions.get(*from_state as usize).unwrap();
        let option = a.get(to_state);
        return if option.is_some() {
            option.unwrap().contains(symbol)
        } else {
            false
        }
    }

    pub fn get_symbols_from_to(&self, from_state: &State, to_state: &State) -> impl Iterator<Item = &Symbol> {
        let vec = self.transitions.get(*from_state as usize).unwrap().get(to_state);
        return if vec.is_some() {
            vec.unwrap().iter()
        } else {
            [].iter()
        }
    }

    pub fn get_from_state<'a>(&'a self, from_state: State) -> impl Iterator<Item=(Symbol, State)> + 'a {
        self.transitions.get(from_state as usize).unwrap().iter()
            .flat_map(|(state, vec)| {vec.iter().map(move |symbol| {(*symbol, *state)})})
    }

    pub fn get_all<'a>(&'a self) -> impl Iterator<Item=(State, Symbol, State)> + 'a {
        return (0..self.transitions.len() as u64)
            .flat_map(move |from_state| {
                self.get_from_state(from_state)
                    .map(move |(symbol, to_state)| {(from_state, symbol, to_state)})
            });
    }
}
