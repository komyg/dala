use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

use crate::{DalaError, ParseError, Position};

#[derive(Parser)]
#[grammar = "dala.pest"]
struct DalaParser;

pub fn parse_dala(str: &str) -> Result<Pairs<Rule>, DalaError> {
    let dala = DalaParser::parse(Rule::dala, str);
    if dala.is_err() {
        return Err(DalaError::ParseError(ParseError {
            pos: { Position { start: 0, end: 0 } },
            message: dala.err().unwrap().to_string(),
        }));
    }

    Ok(dala.unwrap())
}
