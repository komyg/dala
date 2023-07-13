use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{
    common::{DalaExpression, Position, Visitor},
    literal::Str,
    upper::Upper,
};

#[derive(Parser)]
#[grammar = "dala.pest"]
struct DalaParser;

fn parse_expr(pair: Pair<Rule>) -> DalaExpression {
    match pair.as_rule() {
        Rule::string => DalaExpression::Str(Str {
            pos: Position {
                start: pair.as_span().start(),
                end: pair.as_span().end(),
            },
            value: pair.into_inner().next().unwrap().as_str().to_string(),
        }),
        Rule::upper => {
            let child_pair = pair.into_inner().next().unwrap();
            DalaExpression::Upper(Upper {
                pos: Position {
                    start: child_pair.as_span().start(),
                    end: child_pair.as_span().end(),
                },
                child: Box::new(parse_expr(child_pair)),
            })
        }
        Rule::EOI => DalaExpression::None,
        Rule::dala | Rule::inner | Rule::char | Rule::WHITESPACE | Rule::functions => {
            unreachable!()
        }
    }
}

pub fn parse_dala(str: &str) {
    let dala = DalaParser::parse(Rule::dala, str).unwrap();

    let result = dala.map(parse_expr).collect::<Vec<_>>();
    println!("{:#?}", result);

    result.iter().for_each(|expr| {
        let result = expr.eval();
        println!("RESULT {:#?}", result);
    });
}
