use std::fmt;

use super::literal::{Boolean, Num, Str};
use super::upper::Upper;

#[derive(Debug)]
pub enum DalaExpression {
    Str(Str),
    Num(Num),
    Boolean(Boolean),
    Upper(Upper),
    None,
}

impl Visitor for DalaExpression {
    fn eval(&self) -> Result<DalaResult, DalaError> {
        match self {
            DalaExpression::Str(expr) => expr.eval(),
            DalaExpression::Num(expr) => expr.eval(),
            DalaExpression::Boolean(expr) => expr.eval(),
            DalaExpression::Upper(expr) => expr.eval(),
            DalaExpression::None => Err(DalaError("None".to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DalaResult {
    Str(String),
    Num(f64),
    Boolean(bool),
}

pub trait Visitor {
    fn eval(&self) -> Result<DalaResult, DalaError>;
}

impl core::fmt::Debug for dyn Visitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Visitor")
    }
}

#[derive(Debug, Clone)]
pub struct DalaError(String);

impl fmt::Display for DalaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
