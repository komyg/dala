use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::expr::dala::DalaExpression;
use crate::ast::{common::postion::Position, expr::literal::Str, expr::upper::Upper};

#[derive(Parser)]
#[grammar = "dala.pest"]
struct DalaParser;

fn parse_expr(pair: Pair<Rule>) -> DalaExpression {
    match pair.as_rule() {
        Rule::string => DalaExpression::Str(Str {
            pos: Position::new(pair.as_span()),
            value: pair.into_inner().next().unwrap().as_str().to_string(),
        }),
        Rule::upper => DalaExpression::Upper(Upper {
            pos: Position::new(pair.as_span()),
            child: Box::new(parse_expr(pair.into_inner().next().unwrap())),
        }),
        Rule::EOI => DalaExpression::None,
        Rule::dala | Rule::inner | Rule::char | Rule::WHITESPACE | Rule::functions => {
            unreachable!()
        }
    }
}

pub fn parse_dala(str: &str) -> Vec<DalaExpression> {
    let dala = DalaParser::parse(Rule::dala, str).unwrap();

    dala.map(parse_expr)
        .filter(|expr| !matches!(expr, DalaExpression::None))
        .collect::<Vec<_>>()
}
