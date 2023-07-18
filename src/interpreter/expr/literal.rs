use crate::{DalaError, DalaValue, Position};

use super::eval_visitor::EvalVisitor;

#[derive(Debug)]
pub struct Str {
    pub pos: Position,
    pub value: String,
}

#[derive(Debug)]
pub struct Num {
    pub pos: Position,
    pub value: f64,
}

#[derive(Debug)]
pub struct Bool {
    pub pos: Position,
    pub value: bool,
}

impl EvalVisitor for Str {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Str(self.value.clone()))
    }
}

impl EvalVisitor for Num {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Num(self.value))
    }
}

impl EvalVisitor for Bool {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Boolean(self.value))
    }
}
