use super::literal::{Boolean, Num, Str};
use super::upper::Upper;
use crate::{DalaError, DalaValue};
use std::fmt;

#[derive(Debug)]
pub enum DalaExpression {
    Str(Str),
    Num(Num),
    Boolean(Boolean),
    Upper(Upper),
    None,
}

impl Visitor for DalaExpression {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        match self {
            DalaExpression::Str(expr) => expr.eval(),
            DalaExpression::Num(expr) => expr.eval(),
            DalaExpression::Boolean(expr) => expr.eval(),
            DalaExpression::Upper(expr) => expr.eval(),
            DalaExpression::None => Err(DalaError("None".to_string())),
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
