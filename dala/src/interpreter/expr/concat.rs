use super::{eval_visitor::EvalVisitor, DalaExpression};
use crate::{DalaError, DalaValue, Position};

#[derive(Debug, Clone)]
pub struct Concat {
    pub pos: Position,
    pub children: Vec<Box<DalaExpression>>,
}

impl Concat {
    pub fn new(pos: Position, children: Vec<Box<DalaExpression>>) -> Self {
        Self { pos, children }
    }
}

impl EvalVisitor for Concat {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        let evaluated_children = self
            .children
            .iter()
            .map(|child| child.eval())
            .collect::<Result<Vec<DalaValue>, DalaError>>();

        if evaluated_children.is_err() {
            return Err(evaluated_children.unwrap_err());
        }

        let children_values = evaluated_children.unwrap();
        let concat_result =
            children_values
                .into_iter()
                .fold("".to_owned(), |acc, value| match value {
                    DalaValue::Str(value) => acc.to_owned() + &value,
                    DalaValue::Num(value) => acc.to_owned() + &value.to_string(),
                    DalaValue::Boolean(value) => acc.to_owned() + &value.to_string().to_uppercase(),
                });

        Ok(DalaValue::Str(concat_result))
    }
}
