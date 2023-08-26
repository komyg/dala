use std::collections::HashMap;

use pest::iterators::{Pair, Pairs};

use crate::interpreter::expr::{
    comparison::Eq, comparison::Neq, concat::Concat, divide::Divide, if_conditional::IfConditional,
    literal::Bool, literal::Num, literal::Str, mutiply::Multiply, subtract::Subtract, sum::Sum,
    upper::Upper,
};
use crate::{BuildError, DalaError, DalaValue, Position};

use super::expr::DalaExpression;
use super::parser::Rule;

fn build_children(
    pair: Pair<Rule>,
    data: &HashMap<&str, &DalaValue>,
) -> Result<Vec<Box<DalaExpression>>, DalaError> {
    pair.into_inner()
        .map(|inner| build_ast(inner, data).and_then(|expr| Ok(Box::new(expr))))
        .collect::<Result<Vec<Box<DalaExpression>>, DalaError>>()
}

fn build_ast(
    pair: Pair<Rule>,
    data: &HashMap<&str, &DalaValue>,
) -> Result<DalaExpression, DalaError> {
    let pos = Position::new(pair.as_span());
    match pair.as_rule() {
        Rule::EQ => build_children(pair, data)
            .and_then(|children| Ok(DalaExpression::Eq(Eq::new(pos, children)))),
        Rule::NEQ => build_children(pair, data)
            .and_then(|children| Ok(DalaExpression::Neq(Neq::new(pos, children)))),
        Rule::CONCAT => build_children(pair, data)
            .and_then(|children| Ok(DalaExpression::Concat(Concat::new(pos, children)))),
        Rule::DIVIDE => build_children(pair, data)
            .and_then(|children| Ok(DalaExpression::Divide(Divide::new(pos, children)))),
        Rule::IF => build_children(pair, data).and_then(|children| {
            Ok(DalaExpression::IfConditional(IfConditional::new(
                pos, children,
            )))
        }),
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
        Rule::REF => {
            let key = pair.as_str();
            match data.get(key) {
                Some(value) => match value {
                    DalaValue::Str(value) => Ok(DalaExpression::Str(Str::new(pos, value.clone()))),
                    DalaValue::Num(value) => Ok(DalaExpression::Num(Num::new(pos, value.clone()))),
                    DalaValue::Boolean(value) => {
                        Ok(DalaExpression::Bool(Bool::new(pos, value.clone())))
                    }
                },
                None => Err(DalaError::BuildError(BuildError::new(
                    pos,
                    format!("No data found for key: {}", key),
                ))),
            }
        }
        Rule::UPPER => build_children(pair, data).and_then(|children| {
            if children.len() != 1 {
                return Err(DalaError::BuildError(BuildError::new(
                    pos,
                    "Upper must have only one child".to_string(),
                )));
            }

            Ok(DalaExpression::Upper(Upper::new(pos, children[0].clone())))
        }),
        Rule::MULTIPLY => build_children(pair, data)
            .and_then(|children| Ok(DalaExpression::Multiply(Multiply::new(pos, children)))),
        Rule::SUBTRACT => build_children(pair, data)
            .and_then(|children| Ok(DalaExpression::Subtract(Subtract::new(pos, children)))),
        Rule::SUM => build_children(pair, data)
            .and_then(|children| Ok(DalaExpression::Sum(Sum::new(pos, children)))),
        Rule::DALA
        | Rule::INNER
        | Rule::BOOLEAN_ARG
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

pub fn create_ast(
    pairs: Pairs<'_, Rule>,
    data: &HashMap<&str, &DalaValue>,
) -> Vec<Result<DalaExpression, DalaError>> {
    pairs.map(|pair| build_ast(pair, data)).collect()
}
