use std::iter::Peekable;
use nfa::NFA;
use regex::Expression;

#[derive(Debug)]
pub enum Error {
    UnexpectedAlternate(char),
    UnterminatedParenthesis,
    UnexpectedEndOfString,
    UnexpectedEndOfCharacterClass,
}

pub type Result<T> = std::result::Result<T, Error>;

impl NFA<char> {
    pub fn parse_regex(&mut self, s: &str) -> Result<Expression> {
        parse_alternate(self, &mut s.chars().peekable())
    }
}

fn parse_alternate(nfa: &mut NFA<char>, s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Expression> {
    let mut expr = parse_concatenate(nfa, s)?;

    loop {
        match s.peek() {
            Some(&')') | None => break Ok(expr),
            Some(&'|') => {
                s.next();
                let next = parse_alternate(nfa, s)?;
                expr = nfa.alternate(expr, next);
            },
            Some(&e) => break Err(Error::UnexpectedAlternate(e)),
        }
    }
}

fn parse_concatenate(nfa: &mut NFA<char>, s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Expression> {
    let mut expr = parse_kleene_star(nfa, s)?;

    loop {
        match s.peek() {
            Some(&'|') | Some(&')') | None => break Ok(expr),
            _ => {
                let next = parse_kleene_star(nfa, s)?;
                expr = nfa.concatenate(expr, next);
            },
        }
    }
}

fn parse_kleene_star(nfa: &mut NFA<char>, s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Expression> {
    let mut expr = parse_term(nfa, s)?;

    loop {
        match s.peek() {
            Some(&'*') => {
                s.next();
                expr = nfa.kleene_star(expr);
            },
            _ => break Ok(expr),
        }
    }
}

fn parse_term(nfa: &mut NFA<char>, s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Expression> {
    match s.peek() {
        Some(&'(') => {
            s.next();
            let inner = parse_alternate(nfa, s)?;
            if s.next() == Some(')') {
                Ok(inner)
            } else {
                Err(Error::UnterminatedParenthesis)
            }
        },
        Some(&'[') => Ok(parse_character_class(nfa, s)?),
        Some(_) => {
            let c = parse_character(s)?;
            let start = nfa.add_state();
            let end = nfa.add_state();
            nfa.add_edge(start, c.into(), end);
            Ok(Expression {
                start,
                end,
            })
        },
        None => Err(Error::UnexpectedEndOfString),
    }
}

fn parse_character(s: &mut Peekable<impl Iterator<Item = char>>) -> Result<char> {
    match s.next() {
        Some('\\') => {
            match s.next() {
                Some(c) => Ok(c),
                None => Err(Error::UnexpectedEndOfString),
            }
        },
        Some(c) => Ok(c),
        None => Err(Error::UnexpectedEndOfString),
    }
}

fn parse_character_class(nfa: &mut NFA<char>, s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Expression> {
    const ALL_CHARS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

    s.next();

    let invert = match s.peek() {
        Some(&'^') => {
            s.next();
            true
        },
        _ => false,
    };

    let expr = Expression {
        start: nfa.add_state(),
        end: nfa.add_state(),
    };
    loop {
        match s.peek() {
            Some(&']') => {
                s.next();
                break Ok(expr);
            },
            Some(_) => {
                let c = parse_character(s)?;
                match s.peek() {
                    Some(&'-') => {
                        s.next();
                        match s.peek() {
                            Some(&']') => break Err(Error::UnexpectedEndOfCharacterClass),
                            Some(_) => {
                                let d = parse_character(s)?;
                                if !invert {
                                    for x in ALL_CHARS.chars().filter(|&x| (x as u8) >= (c as u8) && (x as u8) <= (d as u8)) {
                                        nfa.add_edge(expr.start, x.into(), expr.end);
                                    }
                                } else {
                                    for x in ALL_CHARS.chars().filter(|&x| (x as u8) < (c as u8) || (x as u8) > (d as u8)) {
                                        nfa.add_edge(expr.start, x.into(), expr.end);
                                    }
                                }
                            },
                            None => break Err(Error::UnexpectedEndOfString),
                        }
                    },
                    Some(&_) => {
                        if !invert {
                            nfa.add_edge(expr.start, c.into(), expr.end);
                        } else {
                            for x in ALL_CHARS.chars().filter(|&x| x != c) {
                                nfa.add_edge(expr.start, x.into(), expr.end);
                            }
                        }
                    },
                    None => break Err(Error::UnexpectedEndOfString),
                }
            },
            None => break Err(Error::UnexpectedEndOfString),
        }
    }
}