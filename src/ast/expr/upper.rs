use super::dala::Visitor;
use crate::{DalaError, DalaValue, Position};

#[derive(Debug)]
pub struct Upper {
    pub pos: Position,
    pub child: Box<dyn Visitor>,
}

impl Visitor for Upper {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        let child_value = self.child.eval();
        match child_value {
            Ok(result) => match result {
                DalaValue::Str(value) => Ok(DalaValue::Str(value.to_uppercase())),
                DalaValue::Num(value) => Ok(DalaValue::Str(value.to_string().to_uppercase())),
                DalaValue::Boolean(value) => Ok(DalaValue::Str(value.to_string().to_uppercase())),
            },
            Err(err) => Err(err),
        }
    }
}
