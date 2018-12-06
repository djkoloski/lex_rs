use std::{
    cmp::Eq,
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};
use dfa::DFA;
use nfa::{Epsilon, NFA};
use parse::Result;

#[derive(Debug)]
pub struct Lexer<T> {
    dfa: DFA<char>,
    state_to_token: HashMap<usize, T>,
    start: usize,
}

impl<'a, T: 'a + Clone> Lexer<T> {
    pub fn from_entries(entries: impl Iterator<Item = (T, &'a str)>) -> Result<Lexer<T>> {
        let mut nfa = NFA::new();
        let root = nfa.add_state();
        let mut terminal_to_token = HashMap::new();
        for (token, regex) in entries {
            let expr = nfa.parse_regex(regex)?;
            nfa.add_edge(root, Epsilon::Epsilon, expr.start);
            terminal_to_token.insert(expr.end, token);
        }
        let (dfa, powerset_to_state, start) = nfa.powerset(root);
        let (dfa, state_to_partition, state_to_powerset) = dfa.minimize_from_nfa(powerset_to_state, |s| terminal_to_token.contains_key(&s));

        let mut state_to_token = HashMap::new();
        for state in dfa.states() {
            let powerset = &state_to_powerset[&state_to_partition[&state][0]];
            if let Some(token) = powerset.iter().find_map(|s| terminal_to_token.get(s)) {
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

impl<T: Clone + Eq + Hash + Display> Lexer<T> {
    pub fn to_cpp(&self) -> String {
        let mut tokens = HashSet::new();
        for (_, token) in &self.state_to_token {
            tokens.insert(token.clone());
        }
        let mut token_enum_values = String::new();
        for token in tokens.drain() {
            token_enum_values = format!("{},\n    {}", token_enum_values, token);
        }

        let mut states = String::new();
        for state in self.dfa.states() {
            states = format!("{}STATE_{}:\n    ", states, state);
            if self.dfa.edges(state).len() == 0 {
                if let Some(token) = self.state_to_token.get(&state) {
                    states = format!("{}result.token_type = TokenType::{}; goto END;\n\n    ", states, token);
                } else {
                    states = format!("{}result.token_type = TokenType::INVALID; goto END;\n\n    ", states);
                }
            } else {
                states = format!("{}switch (*s++) {{\n    ", states);
                for (&symbol, to) in self.dfa.edges(state) {
                    states = format!("{}case {}: goto STATE_{};\n    ", states, symbol as u8, to);
                }
                if let Some(token) = self.state_to_token.get(&state) {
                    states = format!("{}default: result.token_type = TokenType::{}; goto CLEANUP;\n    }}\n\n    ", states, token);
                } else {
                    states = format!("{}default: result.token_type = TokenType::INVALID; goto CLEANUP;\n    }}\n\n    ", states);
                }
            }
        }

        format!(
r#"enum TokenType {{
    INVALID{}
}};

struct Token {{
    const char *begin;
    const char *end;
    TokenType token_type;
}};

Token lex(const char *&s) {{
    Token result;
    result.begin = s;

    goto STATE_{};

    {}CLEANUP:
    --s;
    END:
    result.end = s;
    return result;
}}"#,
            token_enum_values,
            self.start,
            states)
    }
}