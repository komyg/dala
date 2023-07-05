use crate::ast::literal::Literal;
use crate::ast::unary_expression::UnaryExpression;

#[derive(Debug)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub enum DalaExpression {
    Literal(Literal),
    UnaryExpression(UnaryExpression),
    None,
}
