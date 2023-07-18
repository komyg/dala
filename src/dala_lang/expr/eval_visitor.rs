use crate::{DalaError, DalaValue};
use std::fmt;

pub trait EvalVisitor {
    fn eval(&self) -> Result<DalaValue, DalaError>;
}

impl core::fmt::Debug for dyn EvalVisitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Visitor")
    }
}
