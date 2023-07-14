use super::dala::{DalaError, DalaResult, Visitor};
use crate::ast::common::postion::Position;

#[derive(Debug)]
pub struct Upper {
    pub pos: Position,
    pub child: Box<dyn Visitor>,
}

impl Visitor for Upper {
    fn eval(&self) -> Result<DalaResult, DalaError> {
        let child_value = self.child.eval();
        match child_value {
            Ok(result) => match result {
                DalaResult::Str(value) => Ok(DalaResult::Str(value.to_uppercase())),
                DalaResult::Num(value) => Ok(DalaResult::Str(value.to_string().to_uppercase())),
                DalaResult::Boolean(value) => Ok(DalaResult::Str(value.to_string().to_uppercase())),
            },
            Err(err) => Err(err),
        }
    }
}
