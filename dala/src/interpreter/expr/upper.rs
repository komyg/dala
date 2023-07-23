use super::{eval_visitor::EvalVisitor, DalaExpression};
use crate::{DalaError, DalaValue, Position};

#[derive(Debug, Clone)]
pub struct Upper {
    pub pos: Position,
    pub child: Box<DalaExpression>,
}

impl Upper {
    pub fn new(pos: Position, child: Box<DalaExpression>) -> Self {
        Self { pos, child }
    }
}

impl EvalVisitor for Upper {
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
