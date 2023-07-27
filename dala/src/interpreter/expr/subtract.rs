use super::{
    eval_visitor::{eval_children, EvalVisitor},
    macros::create_expr_struct,
    DalaExpression,
};
use crate::{DalaError, DalaValue, Position, RuntimeError};

create_expr_struct!(Subtract);

impl EvalVisitor for Subtract {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        let evaluated_children = eval_children(&self.children);

        if evaluated_children.is_err() {
            return Err(evaluated_children.unwrap_err());
        }

        let calculate = |acc_value: f64, value: DalaValue| match value {
            DalaValue::Str(value) => Err(DalaError::RuntimeError(RuntimeError::new(
                self.pos.clone(),
                format!("Cannot subtract strings, found: {}", value),
            ))),
            DalaValue::Num(value) => Ok(value - acc_value),
            DalaValue::Boolean(value) => Err(DalaError::RuntimeError(RuntimeError::new(
                self.pos.clone(),
                format!("Cannot subtract booleans, found: {}", value),
            ))),
        };

        evaluated_children
            .unwrap()
            .into_iter()
            .rfold(Ok(0.0), |acc, value| {
                acc.and_then(|acc_value| calculate(acc_value, value))
            })
            .and_then(|result| Ok(DalaValue::Num(result)))
    }
}
