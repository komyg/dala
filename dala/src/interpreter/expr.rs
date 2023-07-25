use crate::{DalaError, DalaValue};

pub mod concat;
pub mod eval_visitor;
pub mod literal;
pub mod subtract;
pub mod sum;
pub mod upper;

#[derive(Debug, Clone)]
pub enum DalaExpression {
    Concat(concat::Concat),
    Bool(literal::Bool),
    Num(literal::Num),
    Str(literal::Str),
    Subtract(subtract::Subtract),
    Sum(sum::Sum),
    Upper(upper::Upper),
}

impl eval_visitor::EvalVisitor for DalaExpression {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        match self {
            DalaExpression::Concat(expr) => expr.eval(),
            DalaExpression::Bool(expr) => expr.eval(),
            DalaExpression::Num(expr) => expr.eval(),
            DalaExpression::Str(expr) => expr.eval(),
            DalaExpression::Subtract(expr) => expr.eval(),
            DalaExpression::Sum(expr) => expr.eval(),
            DalaExpression::Upper(expr) => expr.eval(),
        }
    }
}
