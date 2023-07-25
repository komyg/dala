use pest::iterators::{Pair, Pairs};

use crate::interpreter::{
    expr::concat::Concat, expr::literal::Bool, expr::literal::Num, expr::literal::Str,
    expr::sum::Sum, expr::upper::Upper,
};
use crate::{BuildError, DalaError, Position};

use super::expr::DalaExpression;
use super::parser::Rule;

fn build_children(pair: Pair<Rule>) -> Result<Vec<Box<DalaExpression>>, DalaError> {
    pair.into_inner()
        .map(|inner| build_ast(inner).and_then(|expr| Ok(Box::new(expr))))
        .collect::<Result<Vec<Box<DalaExpression>>, DalaError>>()
}

fn build_ast(pair: Pair<Rule>) -> Result<DalaExpression, DalaError> {
    let pos = Position::new(pair.as_span());
    match pair.as_rule() {
        Rule::STRING => match pair.into_inner().next() {
            Some(inner) => Ok(DalaExpression::Str(Str::new(
                pos,
                inner.as_str().to_string(),
            ))),
            None => Err(DalaError::BuildError(BuildError::new(
                pos,
                "Invalid empty string".to_string(),
            ))),
        },
        Rule::NUMBER => Ok(DalaExpression::Num(Num::new(
            pos,
            pair.as_str().parse::<f64>().unwrap(),
        ))),
        Rule::BOOLEAN => Ok(DalaExpression::Bool(Bool::new(
            pos,
            pair.as_str().to_lowercase().parse::<bool>().unwrap(),
        ))),
        Rule::UPPER => build_children(pair).and_then(|children| {
            if children.len() != 1 {
                return Err(DalaError::BuildError(BuildError::new(
                    pos,
                    "Upper must have only one child".to_string(),
                )));
            }

            Ok(DalaExpression::Upper(Upper::new(pos, children[0].clone())))
        }),
        Rule::CONCAT => build_children(pair)
            .and_then(|children| Ok(DalaExpression::Concat(Concat::new(pos, children)))),
        Rule::SUM => build_children(pair)
            .and_then(|children| Ok(DalaExpression::Sum(Sum::new(pos, children)))),
        Rule::DALA
        | Rule::INNER
        | Rule::CHAR
        | Rule::WHITESPACE
        | Rule::FUNCTIONS
        | Rule::eoi
        | Rule::LITERALS
        | Rule::ARG
        | Rule::ARGS
        | Rule::NUM_ARG
        | Rule::NUM_ARGS => {
            unreachable!()
        }
    }
}

pub fn create_ast(pairs: Pairs<'_, Rule>) -> Vec<Result<DalaExpression, DalaError>> {
    pairs.map(build_ast).collect()
}
