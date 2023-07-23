use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

use crate::{DalaError, ParseError};

#[derive(Parser)]
#[grammar = "dala.pest"]
struct DalaParser;

pub fn parse_dala(str: &str) -> Result<Pairs<Rule>, DalaError> {
    let dala = DalaParser::parse(Rule::dala, str);
    if dala.is_err() {
        let err = dala.err().unwrap();
        return Err(DalaError::ParseError(ParseError::new(err.to_string())));
    }

    Ok(dala.unwrap())
}
