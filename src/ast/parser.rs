use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::expr::dala::DalaExpression;
use crate::ast::{expr::literal::Bool, expr::literal::Num, expr::literal::Str, expr::upper::Upper};
use crate::{DalaError, ParseError, Position};

#[derive(Parser)]
#[grammar = "dala.pest"]
struct DalaParser;

fn parse_expr(pair: Pair<Rule>) -> Result<DalaExpression, DalaError> {
    let pos = Position::new(pair.as_span());
    match pair.as_rule() {
        Rule::string => match pair.into_inner().next() {
            Some(inner) => Ok(DalaExpression::Str(Str {
                pos,
                value: inner.as_str().to_string(),
            })),
            None => Err(DalaError::ParseError(ParseError {
                pos,
                message: "empty string".to_string(),
            })),
        },
        Rule::number => match pair.into_inner().next() {
            Some(inner) => Ok(DalaExpression::Num(Num {
                pos,
                value: inner.as_str().parse::<f64>().unwrap(),
            })),
            None => Err(DalaError::ParseError(ParseError {
                pos,
                message: "empty number".to_string(),
            })),
        },
        Rule::boolean => match pair.into_inner().next() {
            Some(inner) => Ok(DalaExpression::Bool(Bool {
                pos,
                value: inner.as_str().parse::<bool>().unwrap(),
            })),
            None => Err(DalaError::ParseError(ParseError {
                pos,
                message: "empty boolean".to_string(),
            })),
        },
        Rule::upper => match pair.into_inner().next() {
            Some(inner) => parse_expr(inner).map_or(
                Err(DalaError::ParseError(ParseError {
                    pos: pos.clone(),
                    message: "empty expression".to_string(),
                })),
                |child| {
                    Ok(DalaExpression::Upper(Upper {
                        pos,
                        child: Box::new(child),
                    }))
                },
            ),
            None => Err(DalaError::ParseError(ParseError {
                pos,
                message: "empty expression".to_string(),
            })),
        },
        Rule::dala | Rule::inner | Rule::char | Rule::WHITESPACE | Rule::functions | Rule::eoi => {
            unreachable!()
        }
    }
}

pub fn parse_dala(str: &str) -> Vec<Result<DalaExpression, DalaError>> {
    let dala = DalaParser::parse(Rule::dala, str).unwrap();

    dala.map(parse_expr).collect::<Vec<_>>()
}
