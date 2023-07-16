use super::literal::{Bool, Num, Str};
use super::upper::Upper;
use crate::{DalaError, DalaValue, Position};
use std::fmt;

#[derive(Debug)]
pub enum DalaExpression {
    Str(Str),
    Num(Num),
    Bool(Bool),
    Upper(Upper),
}

impl Visitor for DalaExpression {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        match self {
            DalaExpression::Str(expr) => expr.eval(),
            DalaExpression::Num(expr) => expr.eval(),
            DalaExpression::Bool(expr) => expr.eval(),
            DalaExpression::Upper(expr) => expr.eval(),
        }
    }
}

pub trait Visitor {
    fn eval(&self) -> Result<DalaValue, DalaError>;
}

impl core::fmt::Debug for dyn Visitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Visitor")
    }
}
