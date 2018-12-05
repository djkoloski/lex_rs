use std::collections::HashMap;
use dfa::DFA;
use nfa::{Eta, NFA};
use parse::Result;

#[derive(Serialize, Deserialize)]
pub struct Lexer<T> {
    dfa: DFA<char>,
    state_to_token: HashMap<usize, T>,
    start: usize,
}

impl<'a, T: 'a + Clone> Lexer<T> {
    pub fn from_entries(entries: impl Iterator<Item = &'a (T, &'a str)>) -> Result<Lexer<T>> {
        let mut nfa = NFA::new();
        let root = nfa.add_state();
        let mut terminal_to_token = HashMap::new();
        for (token, regex) in entries {
            let expr = nfa.parse_regex(regex)?;
            nfa.add_edge(root, Eta::Eta, expr.start);
            terminal_to_token.insert(expr.end, token);
        }
        let (dfa, powerset_to_state, start) = nfa.powerset(root);
        let (dfa, state_to_partition, state_to_powerset) = dfa.minimize_from_nfa(powerset_to_state, |s| terminal_to_token.contains_key(&s));

        let mut state_to_token = HashMap::new();
        for state in dfa.states() {
            let powerset = &state_to_powerset[&state_to_partition[&state][0]];
            if let Some(&token) = powerset.iter().find_map(|s| terminal_to_token.get(s)) {
                state_to_token.insert(state, token.clone());
            }
        }

        let start = state_to_partition.iter()
            .find_map(|(&state, partition)| {
                if partition.contains(&start) {
                    Some(state)
                } else {
                    None
                }
            })
            .unwrap();

        Ok(Lexer {
            dfa,
            state_to_token,
            start,
        })
    }
}