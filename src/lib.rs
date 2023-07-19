use core::fmt;
use pest::Span;

mod interpreter;
use interpreter::expr::eval_visitor::EvalVisitor;
use interpreter::{ast::create_ast, parser::parse_dala};

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
    BuildError(BuildError),
    RuntimeError(RuntimeError),
    ParseError(ParseError),
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub pos: Position,
    pub message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime error: {}, at: {}", self.message, self.pos)
    }
}

#[derive(Debug, Clone)]
pub struct BuildError {
    pub pos: Position,
    pub message: String,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Build error: {}, at: {}", self.message, self.pos)
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

// Public functions

pub fn eval_dala(str: &str) -> Vec<Result<DalaValue, DalaError>> {
    let parsed = parse_dala(str);
    if parsed.is_err() {
        return vec![Err(parsed.unwrap_err())];
    }

    create_ast(parsed.unwrap())
        .into_iter()
        .map(|expr| expr.and_then(|expr| expr.eval()))
        .collect()
}
