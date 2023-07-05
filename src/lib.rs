mod ast;
use ast::parser::parse_dala;

fn parse(str: &str) {
    parse_dala(str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dala() {
        let input = "UPPER(\"abc\")";
        parse(input);
    }
}
