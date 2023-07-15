mod ast;

use crate::ast::expr::dala::Visitor;
use ast::parser::parse_dala;

use core::fmt;

#[derive(Debug, Clone)]
pub enum DalaValue {
    Str(String),
    Num(f64),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct DalaError(pub String);

impl fmt::Display for DalaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn eval_dala(str: &str) -> Vec<Result<DalaValue, DalaError>> {
    let parsed = parse_dala(str);
    parsed.iter().map(|expr| expr.eval()).collect()
}
