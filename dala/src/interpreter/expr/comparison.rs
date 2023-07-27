use super::{eval_visitor::EvalVisitor, DalaExpression};
use crate::{DalaError, DalaValue, Position, RuntimeError};

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

#[derive(Debug, Clone)]
pub struct Neq {
    pub pos: Position,
    pub children: Vec<Box<DalaExpression>>,
}

impl Neq {
    pub fn new(pos: Position, children: Vec<Box<DalaExpression>>) -> Self {
        Self { pos, children }
    }
}

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

macro_rules! create_compare_fn {
    ($func_name:ident, $op:tt) => {
        fn $func_name(left: &DalaValue, right: &DalaValue, pos: &Position) -> Result<DalaValue, DalaError> {
            match (left, right) {
                (DalaValue::Boolean(left), DalaValue::Boolean(right)) => {
                    Ok(DalaValue::Boolean(left $op right))
                }
                (DalaValue::Num(left), DalaValue::Num(right)) => {
                    Ok(DalaValue::Boolean(left $op right))
                }
                (DalaValue::Str(left), DalaValue::Str(right)) => {
                    Ok(DalaValue::Boolean(left $op right))
                }
                (left, right) => Err(DalaError::RuntimeError(RuntimeError::new(
                    pos.clone(),
                    format!("Arguments of the same type, found {} and {}", left, right),
                ))),
            }
        }
    };
}

macro_rules! impl_eval_visitor {
    ($struct_name:ident, $comparison:ident) => {
        impl EvalVisitor for $struct_name {
            fn eval(&self) -> Result<DalaValue, DalaError> {
                get_values(&self.pos, &self.children).and_then(|[left_value, right_value]| {
                    $comparison(&left_value, &right_value, &self.pos)
                })
            }
        }
    };
}

create_compare_fn!(equals, ==);
create_compare_fn!(not_equals, !=);

impl_eval_visitor!(Eq, equals);
impl_eval_visitor!(Neq, not_equals);
