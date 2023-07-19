use crate::{DalaError, DalaValue};

pub mod concat;
pub mod eval_visitor;
pub mod literal;
pub mod upper;

#[derive(Debug)]
pub enum DalaExpression {
    Str(literal::Str),
    Num(literal::Num),
    Bool(literal::Bool),
    Upper(upper::Upper),
    Concat(concat::Concat),
}

impl eval_visitor::EvalVisitor for DalaExpression {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        match self {
            DalaExpression::Upper(expr) => expr.eval(),
            DalaExpression::Bool(expr) => expr.eval(),
            DalaExpression::Num(expr) => expr.eval(),
            DalaExpression::Str(expr) => expr.eval(),
            DalaExpression::Concat(expr) => expr.eval(),
        }
    }
}
