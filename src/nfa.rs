use std::{
    cmp::Eq,
    collections::{
        HashMap,
        HashSet,
    },
    hash::Hash,
    iter,
};
use dfa::DFA;

pub enum Eta<S> {
    Symbol(S),
    Eta,
}

impl<S: Eq + Hash> From<S> for Eta<S> {
    fn from(symbol: S) -> Eta<S> {
        Eta::Symbol(symbol)
    }
}

struct State<S: Eq + Hash> {
    edges: HashMap<S, usize>,
    eta_edges: HashSet<usize>,
}

pub struct NFA<S: Eq + Hash> {
    states: Vec<State<S>>,
}

impl<S: Eq + Hash> NFA<S> {
    pub fn new() -> Self {
        NFA {
            states: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn add_state(&mut self) -> usize {
        let result = self.states.len();
        self.states.push(State {
            edges: Default::default(),
            eta_edges: Default::default(),
        });
        result
    }

    pub fn add_edge(&mut self, from: usize, symbol: Eta<S>, to: usize) -> bool {
        match symbol {
            Eta::Symbol(s) => self.states[from].edges.insert(s, to) == None,
            Eta::Eta => self.states[from].eta_edges.insert(to),
        }
    }

    pub fn states(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }

    pub fn edges(&self, from: usize) -> impl Iterator<Item = (Eta<&S>, &usize)> {
        self.states[from].edges
            .iter()
            .map(|(s, to)| (Eta::Symbol(s), to))
            .chain(
                self.states[from].eta_edges.iter()
                    .map(|to| (Eta::Eta, to)))
    }

    fn eta_set(&self, states: impl Iterator<Item = usize>) -> Vec<usize> {
        let mut result = Vec::new();

        let mut queue: Vec<_> = states.collect();
        while let Some(current) = queue.pop() {
            if let Err(insert_index) = result.binary_search(&current) {
                result.insert(insert_index, current);
                for &to in &self.states[current].eta_edges {
                    queue.push(to);
                }
            }
        }

        result
    }
}

impl<S: Clone + Eq + Hash> NFA<S> {
    pub fn powerset(&self, start: usize) -> (DFA<S>, HashMap<Vec<usize>, usize>, usize) {
        let mut result = DFA::new();
        let mut powerset_to_state = HashMap::new();

        let start_powerset = self.eta_set(iter::once(start));
        let start_state = result.add_state();
        powerset_to_state.insert(start_powerset.clone(), start_state);

        let mut queue = vec![(start_powerset, start_state)];
        while let Some((powerset, state)) = queue.pop() {
            let mut transitions = HashMap::new();

            for &state in &powerset {
                for (symbol, &to) in &self.states[state].edges {
                    transitions.entry(symbol).or_insert(HashSet::new()).insert(to);
                }
            }

            for (symbol, next_set) in transitions.drain() {
                let next_powerset = self.eta_set(next_set.iter().cloned());
                let next_state = powerset_to_state.entry(next_powerset.clone()).or_insert_with(|| {
                    let state = result.add_state();
                    queue.push((next_powerset, state));
                    state
                });
                result.add_edge(state, symbol.clone(), *next_state);
            }
        }

        (result, powerset_to_state, start_state)
    }
}