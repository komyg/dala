use dala::{eval_dala, DalaError};

#[test]
fn test_invalid_syntax() {
    let result = eval_dala("UPPER(\"abc)");
    assert_eq!(result.len(), 1);

    let DalaError::ParseError(parse_error) = result[0].as_ref().unwrap_err() else { panic!("Not a parse error") };
    assert_eq!(parse_error.message.len() > 0, true);
}
