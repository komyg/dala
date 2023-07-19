use pest::iterators::{Pair, Pairs};

use crate::interpreter::{
    expr::concat::Concat, expr::literal::Bool, expr::literal::Num, expr::literal::Str,
    expr::upper::Upper,
};
use crate::{BuildError, DalaError, Position};

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
            None => Err(DalaError::BuildError(BuildError {
                pos,
                message: "empty string".to_string(),
            })),
        },
        Rule::number => Ok(DalaExpression::Num(Num {
            pos,
            value: pair.as_str().parse::<f64>().unwrap(),
        })),
        Rule::boolean => Ok(DalaExpression::Bool(Bool {
            pos,
            value: pair.as_str().to_lowercase().parse::<bool>().unwrap(),
        })),
        Rule::upper => match pair.into_inner().next() {
            Some(inner) => build_ast(inner).map_or(
                Err(DalaError::BuildError(BuildError {
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
            None => Err(DalaError::BuildError(BuildError {
                pos,
                message: "empty expression".to_string(),
            })),
        },
        Rule::concat => {
            let children = pair
                .into_inner()
                .map(|inner| build_ast(inner))
                .collect::<Result<Vec<DalaExpression>, DalaError>>();

            if children.is_err() {
                return Err(children.unwrap_err());
            }
            let boxed_children = children
                .unwrap()
                .into_iter()
                .map(|child| Box::new(child))
                .collect::<Vec<Box<DalaExpression>>>();

            Ok(DalaExpression::Concat(Concat {
                pos,
                children: boxed_children,
            }))
        }
        // Rule::concat => match pair.into_inner().next() {
        //     Some(inner) => build_ast(inner).map_or(
        //         Err(DalaError::BuildError(BuildError {
        //             pos: pos.clone(),
        //             message: "empty expression".to_string(),
        //         })),
        //         |child| {
        //             Ok(DalaExpression::Concat(
        //                 crate::interpreter::expr::concat::Concat {
        //                     pos,
        //                     children: vec![Box::new(child)],
        //                 },
        //             ))
        //         },
        //     ),
        //     None => Err(DalaError::BuildError(BuildError {
        //         pos,
        //         message: "empty expression".to_string(),
        //     })),
        // },
        Rule::dala
        | Rule::inner
        | Rule::char
        | Rule::WHITESPACE
        | Rule::functions
        | Rule::eoi
        | Rule::literals
        | Rule::arg
        | Rule::args => {
            unreachable!()
        }
    }
}

pub fn create_ast(pairs: Pairs<'_, Rule>) -> Vec<Result<DalaExpression, DalaError>> {
    pairs.map(build_ast).collect()
}
