use crate::ast::literal::Literal;

use crate::ast::common::{DalaExpression, Position};

#[derive(Debug)]
pub struct Upper {
    pub pos: Position,
    pub child: Box<DalaExpression>,
}
