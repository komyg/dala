use crate::ast::common::postion::Position;

use super::dala::{DalaError, DalaResult, Visitor};

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
pub struct Boolean {
    pub pos: Position,
    pub value: bool,
}

impl Visitor for Str {
    fn eval(&self) -> Result<DalaResult, DalaError> {
        Ok(DalaResult::Str(self.value.clone()))
    }
}

impl Visitor for Num {
    fn eval(&self) -> Result<DalaResult, DalaError> {
        Ok(DalaResult::Num(self.value))
    }
}

impl Visitor for Boolean {
    fn eval(&self) -> Result<DalaResult, DalaError> {
        Ok(DalaResult::Boolean(self.value))
    }
}
