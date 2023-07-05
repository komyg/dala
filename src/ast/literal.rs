use crate::ast::common::Position;

#[derive(Debug)]
pub enum Literal {
    Str(Str),
    Number(Number),
    Boolean(Boolean),
}

#[derive(Debug)]
pub struct Str {
    pub pos: Position,
    pub value: String,
}

#[derive(Debug)]
pub struct Number {
    pub pos: Position,
    pub value: f64,
}

#[derive(Debug)]
pub struct Boolean {
    pub pos: Position,
    pub value: bool,
}
