use crate::{DalaError, DalaValue};

pub mod concat;
pub mod divide;
pub mod eval_visitor;
pub mod literal;
pub mod mutiply;
pub mod subtract;
pub mod sum;
pub mod upper;

#[derive(Debug, Clone)]
pub enum DalaExpression {
    Concat(concat::Concat),
    Divide(divide::Divide),
    Bool(literal::Bool),
    Num(literal::Num),
    Str(literal::Str),
    Multiply(mutiply::Multiply),
    Subtract(subtract::Subtract),
    Sum(sum::Sum),
    Upper(upper::Upper),
}

impl eval_visitor::EvalVisitor for DalaExpression {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        match self {
            DalaExpression::Concat(expr) => expr.eval(),
            DalaExpression::Divide(expr) => expr.eval(),
            DalaExpression::Bool(expr) => expr.eval(),
            DalaExpression::Num(expr) => expr.eval(),
            DalaExpression::Str(expr) => expr.eval(),
            DalaExpression::Multiply(expr) => expr.eval(),
            DalaExpression::Subtract(expr) => expr.eval(),
            DalaExpression::Sum(expr) => expr.eval(),
            DalaExpression::Upper(expr) => expr.eval(),
        }
    }
}
