mod ast;
use crate::ast::expr::dala::Visitor;
use ast::parser::parse_dala;
use pest::Span;

use core::fmt;

#[derive(Debug, Clone)]
pub enum DalaValue {
    Str(String),
    Num(f64),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn new(pair: Span) -> Self {
        Self {
            start: pair.start(),
            end: pair.end(),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.start, self.end)
    }
}

// Error Types

#[derive(Debug, Clone)]
pub enum DalaError {
    EvalError(EvalError),
    ParseError(ParseError),
}

#[derive(Debug, Clone)]
pub struct EvalError {
    pub pos: Position,
    pub message: String,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Evaluation error: {}, at: {}", self.message, self.pos)
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub pos: Position,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}, at: {}", self.message, self.pos)
    }
}

// Public functions

pub fn eval_dala(str: &str) -> Vec<Result<DalaValue, DalaError>> {
    let parsed = parse_dala(str);
    parsed.iter().map(|expr| expr.eval()).collect()
}
