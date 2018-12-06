use std::{
    cmp::Eq,
    hash::Hash,
};
use nfa::{Epsilon, NFA};

pub struct Expression {
    pub start: usize,
    pub end: usize,
}

impl<S: Eq + Hash> NFA<S> {
    pub fn add_expression(&mut self, symbol: impl Into<Epsilon<S>>) -> Expression {
        let result = Expression {
            start: self.add_state(),
            end: self.add_state(),
        };
        self.add_edge(result.start, symbol, result.end);
        result
    }

    pub fn alternate(&mut self, left: Expression, right: Expression) -> Expression {
        let start = self.add_state();
        let end = self.add_state();
        self.add_edge(start, Epsilon::Epsilon, left.start);
        self.add_edge(start, Epsilon::Epsilon, right.start);
        self.add_edge(left.end, Epsilon::Epsilon, end);
        self.add_edge(right.end, Epsilon::Epsilon, end);
        Expression {
            start,
            end,
        }
    }

    pub fn concatenate(&mut self, left: Expression, right: Expression) -> Expression {
        self.add_edge(left.end, Epsilon::Epsilon, right.start);
        Expression {
            start: left.start,
            end: right.end,
        }
    }

    pub fn zero_or_more(&mut self, expr: Expression) -> Expression {
        let start = self.add_state();
        let end = self.add_state();
        self.add_edge(start, Epsilon::Epsilon, expr.start);
        self.add_edge(start, Epsilon::Epsilon, end);
        self.add_edge(expr.end, Epsilon::Epsilon, expr.start);
        self.add_edge(expr.end, Epsilon::Epsilon, end);
        Expression {
            start,
            end,
        }
    }

    pub fn one_or_more(&mut self, expr: Expression) -> Expression {
        let start = self.add_state();
        let end = self.add_state();
        self.add_edge(start, Epsilon::Epsilon, expr.start);
        self.add_edge(expr.end, Epsilon::Epsilon, expr.start);
        self.add_edge(expr.end, Epsilon::Epsilon, end);
        Expression {
            start,
            end,
        }
    }

    pub fn zero_or_one(&mut self, expr: Expression) -> Expression {
        let start = self.add_state();
        let end = self.add_state();
        self.add_edge(start, Epsilon::Epsilon, expr.start);
        self.add_edge(start, Epsilon::Epsilon, end);
        self.add_edge(expr.end, Epsilon::Epsilon, end);
        Expression {
            start,
            end,
        }
    }
}