use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{
    common::{DalaExpression, Position},
    literal::{Literal, Str},
    unary_expression::UnaryExpression,
    upper::Upper,
};

#[derive(Parser)]
#[grammar = "dala.pest"]
struct DalaParser;

fn parse_expr(pair: Pair<Rule>) -> DalaExpression {
    // println!("{:#?}", pair);
    match pair.as_rule() {
        Rule::string => DalaExpression::Literal(Literal::Str(Str {
            pos: Position {
                start: pair.as_span().start(),
                end: pair.as_span().end(),
            },
            value: pair.into_inner().next().unwrap().as_str().to_string(),
        })),
        Rule::upper => {
            let child_pair = pair.into_inner().next().unwrap();
            DalaExpression::UnaryExpression(UnaryExpression::Upper(Upper {
                pos: Position {
                    start: child_pair.as_span().start(),
                    end: child_pair.as_span().end(),
                },
                child: Box::new(parse_expr(child_pair)),
            }))
        }
        Rule::dala | Rule::EOI | Rule::inner | Rule::char | Rule::WHITESPACE | Rule::functions => {
            // println!("{:#?}", pair);
            DalaExpression::None
        }
    }
}

pub fn parse_dala(str: &str) {
    let dala = DalaParser::parse(Rule::dala, str).unwrap();

    //println!("{:#?}", dala);

    let result = dala.map(parse_expr).collect::<Vec<_>>();
    println!("{:#?}", result);
}
