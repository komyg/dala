use super::{eval_visitor::EvalVisitor, macros::create_expr_struct, DalaExpression};
use crate::{DalaError, DalaValue, Position};

create_expr_struct!(Concat);

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
