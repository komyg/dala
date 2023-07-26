use super::{eval_visitor::EvalVisitor, DalaExpression};
use crate::{DalaError, DalaValue, Position, RuntimeError};

#[derive(Debug, Clone)]
pub struct IfConditional {
    pub pos: Position,
    pub children: Vec<Box<DalaExpression>>,
}

impl IfConditional {
    pub fn new(pos: Position, children: Vec<Box<DalaExpression>>) -> Self {
        Self { pos, children }
    }

    fn eval_conditional(&self, condition: &Box<DalaExpression>) -> Result<bool, DalaError> {
        let result = condition.eval();
        result.and_then(|value| match value {
            DalaValue::Boolean(b) => Ok(b),
            DalaValue::Num(num) => Err(DalaError::RuntimeError(RuntimeError::new(
                self.pos.clone(),
                format!(
                    "If conditional condition must be a boolean, found number: {}",
                    num
                ),
            ))),
            DalaValue::Str(str) => Err(DalaError::RuntimeError(RuntimeError::new(
                self.pos.clone(),
                format!(
                    "If conditional condition must be a boolean, found string {}",
                    str
                ),
            ))),
        })
    }
}

impl EvalVisitor for IfConditional {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        let [condition, if_true, if_false] = match self.children.as_slice() {
            [condition, if_true, if_false] => [condition, if_true, if_false],
            _ => {
                return Err(DalaError::RuntimeError(RuntimeError::new(
                    self.pos.clone(),
                    "If conditional must have three arguments".to_string(),
                )))
            }
        };

        self.eval_conditional(condition)
            .and_then(|condition_result| {
                if condition_result {
                    if_true.eval()
                } else {
                    if_false.eval()
                }
            })
    }
}
