use crate::{DalaError, DalaValue};
use std::fmt;

use super::DalaExpression;

pub trait EvalVisitor {
    fn eval(&self) -> Result<DalaValue, DalaError>;
}

impl core::fmt::Debug for dyn EvalVisitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Visitor")
    }
}

pub fn eval_children(children: &Vec<Box<DalaExpression>>) -> Result<Vec<DalaValue>, DalaError> {
    children
        .iter()
        .map(|child| child.eval())
        .collect::<Result<Vec<DalaValue>, DalaError>>()
}
