use pest::iterators::{Pair, Pairs};

use crate::interpreter::{
    expr::literal::Bool, expr::literal::Num, expr::literal::Str, expr::upper::Upper,
};
use crate::{DalaError, ParseError, Position};

use super::expr::DalaExpression;
use super::parser::Rule;

fn build_ast(pair: Pair<Rule>) -> Result<DalaExpression, DalaError> {
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
            Some(inner) => build_ast(inner).map_or(
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
        Rule::dala
        | Rule::inner
        | Rule::char
        | Rule::WHITESPACE
        | Rule::functions
        | Rule::eoi
        | Rule::literals => {
            unreachable!()
        }
    }
}

pub fn create_ast(pairs: Pairs<'_, Rule>) -> Vec<Result<DalaExpression, DalaError>> {
    pairs.map(build_ast).collect()
}
