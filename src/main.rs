mod dfa;
mod lexer;
mod nfa;
mod parse;
mod regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use lexer::Lexer;

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", generate_lexer_from_entries(contents.as_str()));
}

fn generate_lexer_from_entries(entries: &str) -> String {
    let lexer = Lexer::from_entries(entries.lines().map(|line| {
        let (token, regex) = line.split_at(line.find(char::is_whitespace).unwrap());
        (token, regex.trim_left())
    })).unwrap();
    format!("{:?}", lexer)
}