use super::{eval_visitor::EvalVisitor, DalaExpression};
use crate::{DalaError, DalaValue, Position, RuntimeError};

fn get_values(
    pos: &Position,
    children: &Vec<Box<DalaExpression>>,
) -> Result<[DalaValue; 2], DalaError> {
    let [left, right] = match children.as_slice() {
        [left, right] => [left, right],
        _ => {
            return Err(DalaError::RuntimeError(RuntimeError::new(
                pos.clone(),
                "Eq must have two arguments".to_string(),
            )))
        }
    };

    left.eval().and_then(|left_value| {
        right
            .eval()
            .and_then(|right_value| Ok([left_value, right_value]))
    })
}

#[derive(Debug, Clone)]
pub struct Eq {
    pub pos: Position,
    pub children: Vec<Box<DalaExpression>>,
}

impl Eq {
    pub fn new(pos: Position, children: Vec<Box<DalaExpression>>) -> Self {
        Self { pos, children }
    }
}

impl EvalVisitor for Eq {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        get_values(&self.pos, &self.children).and_then(|[left_value, right_value]| {
            match (left_value, right_value) {
                (DalaValue::Boolean(left), DalaValue::Boolean(right)) => {
                    Ok(DalaValue::Boolean(left == right))
                }
                (DalaValue::Num(left), DalaValue::Num(right)) => {
                    Ok(DalaValue::Boolean(left == right))
                }
                (DalaValue::Str(left), DalaValue::Str(right)) => {
                    Ok(DalaValue::Boolean(left == right))
                }
                (left, right) => Err(DalaError::RuntimeError(RuntimeError::new(
                    self.pos.clone(),
                    format!(
                        "Eq must have two arguments of the same type, found {} and {}",
                        left, right
                    ),
                ))),
            }
        })
    }
}
