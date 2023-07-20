use crate::{DalaError, DalaValue, Position};

use super::eval_visitor::EvalVisitor;

#[derive(Debug, Clone)]
pub struct Str {
    pub pos: Position,
    pub value: String,
}

impl Str {
    pub fn new(pos: Position, value: String) -> Self {
        Self { pos, value }
    }
}

impl EvalVisitor for Str {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Str(self.value.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct Num {
    pub pos: Position,
    pub value: f64,
}

impl Num {
    pub fn new(pos: Position, value: f64) -> Self {
        Self { pos, value }
    }
}

impl EvalVisitor for Num {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Num(self.value))
    }
}

#[derive(Debug, Clone)]
pub struct Bool {
    pub pos: Position,
    pub value: bool,
}

impl Bool {
    pub fn new(pos: Position, value: bool) -> Self {
        Self { pos, value }
    }
}

impl EvalVisitor for Bool {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Boolean(self.value))
    }
}
