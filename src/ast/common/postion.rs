use pest::Span;

#[derive(Debug, Clone)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn new(pair: Span) -> Self {
        Self {
            start: pair.start(),
            end: pair.end(),
        }
    }
}
