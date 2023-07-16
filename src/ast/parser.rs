use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::expr::dala::DalaExpression;
use crate::ast::{expr::literal::Bool, expr::literal::Num, expr::literal::Str, expr::upper::Upper};
use crate::DalaError;
use crate::Position;

#[derive(Parser)]
#[grammar = "dala.pest"]
struct DalaParser;

fn parse_expr(pair: Pair<Rule>) -> DalaExpression {
    match pair.as_rule() {
        Rule::string => {
            // let inner = pair.into_inner().next();
            // if inner.is_none() {
            //     return Err(DalaError::ParseError(Position::new(pair.as_span())));
            // }
            // let pair = inner.unwrap();
            DalaExpression::Str(Str {
                pos: Position::new(pair.as_span()),
                value: pair.into_inner().next().unwrap().as_str().to_string(),
            })
        }
        Rule::number => DalaExpression::Num(Num {
            pos: Position::new(pair.as_span()),
            value: pair.as_str().parse::<f64>().unwrap(),
        }),
        Rule::boolean => DalaExpression::Bool(Bool {
            pos: Position::new(pair.as_span()),
            value: pair.as_str().parse::<bool>().unwrap(),
        }),
        Rule::upper => DalaExpression::Upper(Upper {
            pos: Position::new(pair.as_span()),
            child: Box::new(parse_expr(pair.into_inner().next().unwrap())),
        }),
        Rule::dala | Rule::inner | Rule::char | Rule::WHITESPACE | Rule::functions | Rule::eoi => {
            unreachable!()
        }
    }
}

pub fn parse_dala(str: &str) -> Vec<DalaExpression> {
    let dala = DalaParser::parse(Rule::dala, str).unwrap();

    dala.map(parse_expr).collect::<Vec<_>>()
}
