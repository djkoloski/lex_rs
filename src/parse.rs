use std::{
    collections::HashSet,
    iter::Peekable,
};
use nfa::NFA;
use regex::Expression;

#[derive(Debug)]
pub enum Error {
    UnexpectedNonAlternator,
    UnterminatedParenthesis,
    UnexpectedEndOfString,
    UnexpectedMetacharacter,
    InvalidHexCode,
    InvalidBase26Code,
    InvalidEscapeSequence,
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
            Some(')') | None => break Ok(expr),
            Some('|') => {
                s.next();
                let next = parse_alternate(nfa, s)?;
                expr = nfa.alternate(expr, next);
            },
            Some(_) => break Err(Error::UnexpectedNonAlternator),
        }
    }
}

fn parse_concatenate(nfa: &mut NFA<char>, s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Expression> {
    let mut expr = parse_quantifiers(nfa, s)?;

    loop {
        match s.peek() {
            Some('|') | Some(')') | None => break Ok(expr),
            _ => {
                let next = parse_quantifiers(nfa, s)?;
                expr = nfa.concatenate(expr, next);
            },
        }
    }
}

fn parse_quantifiers(nfa: &mut NFA<char>, s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Expression> {
    let mut expr = parse_term(nfa, s)?;

    loop {
        match s.peek() {
            Some('*') => {
                s.next();
                expr = nfa.zero_or_more(expr);
            },
            Some('+') => {
                s.next();
                expr = nfa.one_or_more(expr);
            },
            Some('?') => {
                s.next();
                expr = nfa.zero_or_one(expr);
            },
            _ => break Ok(expr),
        }
    }
}

fn parse_term(nfa: &mut NFA<char>, s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Expression> {
    match s.peek() {
        Some('(') => {
            s.next();
            let inner = parse_alternate(nfa, s)?;
            if s.next() == Some(')') {
                Ok(inner)
            } else {
                Err(Error::UnterminatedParenthesis)
            }
        },
        Some('[') => Ok(parse_character_class(nfa, s)?),
        Some(_) => Ok(parse_term_character(nfa, s)?),
        None => Err(Error::UnexpectedEndOfString),
    }
}

fn parse_term_character(nfa: &mut NFA<char>, s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Expression> {
    match s.next() {
        Some('\\') => {
            match s.next() {
                Some('x') => {
                    if let (Some(x1), Some(x2)) = (s.next(), s.next()) {
                        Ok(nfa.add_expression(hex_to_char(x1, x2)?))
                    } else {
                        Err(Error::UnexpectedEndOfString)
                    }
                },
                Some('n') => Ok(nfa.add_expression('\n')),
                Some('r') => Ok(nfa.add_expression('\r')),
                Some('t') => Ok(nfa.add_expression('\t')),
                Some('f') => Ok(nfa.add_expression('\x0c')),
                Some('v') => Ok(nfa.add_expression('\x0b')),
                Some('c') => {
                    if let Some(x) = s.next() {
                        let code = (base_26_value(x)? + 1) as char;
                        Ok(nfa.add_expression(code))
                    } else {
                        Err(Error::UnexpectedEndOfString)
                    }
                },
                Some('0') => Ok(nfa.add_expression('\0')),
                Some('[') => Ok(nfa.add_expression('[')),
                Some('\\') => Ok(nfa.add_expression('\\')),
                Some('^') => Ok(nfa.add_expression('^')),
                Some('$') => Ok(nfa.add_expression('$')),
                Some('.') => Ok(nfa.add_expression('.')),
                Some('|') => Ok(nfa.add_expression('|')),
                Some('?') => Ok(nfa.add_expression('?')),
                Some('*') => Ok(nfa.add_expression('*')),
                Some('+') => Ok(nfa.add_expression('+')),
                Some('(') => Ok(nfa.add_expression('(')),
                Some(')') => Ok(nfa.add_expression(')')),
                Some('{') => Ok(nfa.add_expression('{')),
                Some('}') => Ok(nfa.add_expression('}')),
                Some(_) => Err(Error::InvalidEscapeSequence),
                None => Err(Error::UnexpectedEndOfString),
            }
        },
        Some('.') => {
            let expr = Expression {
                start: nfa.add_state(),
                end: nfa.add_state(),
            };
            for x in (0u8..=255u8).map(|x| x as char).filter(|&x| x != '\r' && x != '\n') {
                nfa.add_edge(expr.start, x, expr.end);
            }
            Ok(expr)
        }
        Some('[') | Some('$') | Some('|') | Some('?') | Some('*') | Some('+') | Some('(') | Some(')') => Err(Error::UnexpectedMetacharacter),
        Some(c) => Ok(nfa.add_expression(c)),
        None => Err(Error::UnexpectedEndOfString),
    }
}

fn hex_to_char(x1: char, x2: char) -> Result<char> {
    Ok((hex_value(x1)? << 4 | hex_value(x2)?) as char)
}

fn hex_value(x: char) -> Result<u8> {
    let x = x as u8;
    if x >= '0' as u8 && x <= '9' as u8 {
        Ok(x - '0' as u8)
    } else if x >= 'A' as u8 && x <= 'F' as u8 {
        Ok(x - 'A' as u8)
    } else if x >= 'a' as u8 && x <= 'f' as u8 {
        Ok(x - 'a' as u8)
    } else {
        Err(Error::InvalidHexCode)
    }
}

fn base_26_value(x: char) -> Result<u8> {
    let x = x as u8;
    if x >= 'A' as u8 && x <= 'Z' as u8 {
        Ok(x - 'A' as u8)
    } else if x >= 'a' as u8 && x <= 'z' as u8 {
        Ok(x - 'a' as u8)
    } else {
        Err(Error::InvalidBase26Code)
    }
}

fn parse_character_class(nfa: &mut NFA<char>, s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Expression> {
    s.next();

    let invert = match s.peek() {
        Some('^') => {
            s.next();
            true
        },
        _ => false,
    };

    let mut class = HashSet::new();

    match s.peek() {
        Some(']') => {
            s.next();
            class.insert(']');
        },
        Some('-') => {
            s.next();
            class.insert('-');
        },
        _ => (),
    }

    loop {
        match s.peek() {
            Some(']') => {
                s.next();
                break;
            },
            Some(_) => {
                let c = parse_character_class_character(s)?;
                match s.peek() {
                    Some('-') => {
                        s.next();
                        match s.peek() {
                            Some(']') => {
                                class.insert(c);
                                class.insert('-');
                            },
                            Some(_) => {
                                let d = parse_character_class_character(s)?;
                                for x in c as u8..=d as u8 {
                                    class.insert(x as char);
                                }
                            },
                            None => return Err(Error::UnexpectedEndOfString),
                        }
                    },
                    Some(_) => {
                        class.insert(c);
                    },
                    None => return Err(Error::UnexpectedEndOfString),
                }
            },
            None => return Err(Error::UnexpectedEndOfString),
        }
    }

    let expr = Expression {
        start: nfa.add_state(),
        end: nfa.add_state(),
    };

    if !invert {
        for &x in class.iter() {
            nfa.add_edge(expr.start, x, expr.end);
        }
    } else {
        for x in (0u8..=255u8).map(|x| x as char).filter(|x| !class.contains(x)) {
            nfa.add_edge(expr.start, x, expr.end);
        }
    }

    Ok(expr)
}

fn parse_character_class_character(s: &mut Peekable<impl Iterator<Item = char>>) -> Result<char> {
    match s.next() {
        Some('\\') => {
            match s.next() {
                Some('x') => {
                    if let (Some(x1), Some(x2)) = (s.next(), s.next()) {
                        Ok(hex_to_char(x1, x2)?)
                    } else {
                        Err(Error::UnexpectedEndOfString)
                    }
                },
                Some('n') => Ok('\n'),
                Some('r') => Ok('\r'),
                Some('t') => Ok('\t'),
                Some('f') => Ok('\x0c'),
                Some('v') => Ok('\x0b'),
                Some('c') => {
                    if let Some(x) = s.next() {
                        let code = (base_26_value(x)? + 1) as char;
                        Ok(code)
                    } else {
                        Err(Error::UnexpectedEndOfString)
                    }
                },
                Some('0') => Ok('\0'),
                Some(']') => Ok(']'),
                Some('\\') => Ok('\\'),
                Some('^') => Ok('^'),
                Some('-') => Ok('-'),
                Some(_) => Err(Error::InvalidEscapeSequence),
                None => Err(Error::UnexpectedEndOfString),
            }
        },
        Some(c) => Ok(c),
        None => Err(Error::UnexpectedEndOfString),
    }
}