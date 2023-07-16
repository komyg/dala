use crate::{DalaError, DalaValue, Position};

use super::dala::Visitor;

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

impl Visitor for Str {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Str(self.value.clone()))
    }
}

impl Visitor for Num {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Num(self.value))
    }
}

impl Visitor for Bool {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Boolean(self.value))
    }
}
