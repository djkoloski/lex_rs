extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

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

fn generate_lexer_from_entries(entries_str: &str) -> String {
    let mut entries = Vec::new();
    let mut iter = entries_str.lines().peekable();
    while iter.peek() != None {
        entries.push((iter.next().unwrap(), iter.next().unwrap()));
    }
    let lexer = Lexer::from_entries(entries.iter()).unwrap();
    serde_json::to_string(&lexer).unwrap()
}