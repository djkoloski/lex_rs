use std::{
    cmp::Eq,
    collections::HashMap,
    hash::Hash,
};

#[derive(Serialize, Deserialize)]
struct State<S: Eq + Hash> {
    edges: HashMap<S, usize>,
}

#[derive(Serialize, Deserialize)]
pub struct DFA<S: Eq + Hash> {
    states: Vec<State<S>>,
}

impl<S: Eq + Hash> DFA<S> {
    pub fn new() -> Self {
        DFA {
            states: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn add_state(&mut self) -> usize {
        let result = self.states.len();
        self.states.push(State {
            edges: Default::default()
        });
        result
    }

    pub fn add_edge(&mut self, from: usize, symbol: S, to: usize) -> bool {
        self.states[from].edges.insert(symbol, to) == None
    }

    pub fn states(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }

    pub fn edges(&self, from: usize) -> impl Iterator<Item = (&S, &usize)> {
        self.states[from].edges.iter()
    }
}

impl<S: Clone + Eq + Hash> DFA<S> {
    pub fn minimize_from_nfa(&self, mut powerset_to_state: HashMap<Vec<usize>, usize>, is_terminal: impl Fn(usize) -> bool) -> (DFA<S>, HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {
        let mut state_to_powerset = HashMap::new();
        for (powerset, state) in powerset_to_state.drain() {
            state_to_powerset.insert(state, powerset);
        }

        let (dfa, state_to_partition) = {
            let terminal_for = |state| state_to_powerset[&state].iter().cloned().find(|&state| is_terminal(state));
            self.minimize(|a, b| terminal_for(a) == terminal_for(b))
        };
        (dfa, state_to_partition, state_to_powerset)
    }

    pub fn minimize(&self, compare_terminal: impl Fn(usize, usize) -> bool) -> (DFA<S>, HashMap<usize, Vec<usize>>) {
        let mut partitions = vec![self.states().collect::<Vec<_>>()];
        let mut state_to_partition_index = HashMap::new();
        for state in self.states() {
            state_to_partition_index.insert(state, 0);
        }

        let mut partition_index = 0;
        while partition_index < partitions.len() {
            let mut different = Vec::new();

            {
                let partition = &mut partitions[partition_index];
                let leader = partition[0];
                let leader_state = &self.states[leader];

                let mut state_index = 1;
                while state_index < partition.len() {
                    let follower = partition[state_index];
                    let follower_state = &self.states[follower];

                    let equivalent = compare_terminal(leader, follower)
                        && leader_state.edges.len() == follower_state.edges.len()
                        && leader_state.edges
                            .iter()
                            .all(|(symbol, leader_to)| {
                                if let Some(follower_to) = follower_state.edges.get(symbol) {
                                    state_to_partition_index.get(leader_to) == state_to_partition_index.get(follower_to)
                                } else {
                                    false
                                }
                            });

                    if equivalent {
                        state_index += 1;
                    } else {
                        partition.remove(state_index);
                        different.push(follower);
                    }
                }
            }

            if different.len() > 0 {
                for &state in &different {
                    state_to_partition_index.insert(state, partitions.len());
                }
                partitions.push(different);
            } else {
                partition_index += 1;
            }
        }

        let mut result = DFA::new();
        let mut partition_states = Vec::new();
        for _ in 0..partitions.len() {
            partition_states.push(result.add_state());
        }
        let mut state_to_partition = HashMap::new();
        for (partition_index, partition) in partitions.drain(..).enumerate() {
            for (symbol, to) in &self.states[partition[0]].edges {
                result.add_edge(partition_states[partition_index], symbol.clone(), partition_states[state_to_partition_index[to]]);
            }
            state_to_partition.insert(partition_states[partition_index], partition);
        }

        (result, state_to_partition)
    }
}