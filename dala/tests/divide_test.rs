use std::collections::HashMap;

use dala::{eval, eval_with_data, DalaError, DalaValue};

#[test]
fn test_divide() {
    let result = eval("DIVIDE(10, 2)");
    assert_eq!(result.len(), 1);

    let DalaValue::Num(value) = result[0].as_ref().unwrap() else { panic!("Not a number") };
    assert_eq!(value.to_owned(), 5.0);
}

#[test]
fn test_divide_with_single_arg() {
    let result = eval("DIVIDE(1)");
    assert_eq!(result.len(), 1);

    let DalaError::ParseError(err) = result[0].as_ref().unwrap_err() else { panic!("Not a parse error") };
    assert!(err.message.len() > 0);
}

#[test]
fn test_divide_runtime_error_with_str_arg() {
    let result = eval("DIVIDE(1, UPPER(\"2\"))");
    assert_eq!(result.len(), 1);

    let DalaError::RuntimeError(err) = result[0].as_ref().unwrap_err() else { panic!("Not a runtime error") };
    assert_eq!(err.message, "Cannot divide strings, found: 2");
}

#[test]
fn test_divide_with_data() {
    let result = eval_with_data(
        "DIVIDE($foo, $bar)",
        &HashMap::from([
            ("$foo", &DalaValue::Num(10.0)),
            ("$bar", &DalaValue::Num(2.0)),
        ]),
    );
    assert_eq!(result.len(), 1);

    let DalaValue::Num(value) = result[0].as_ref().unwrap() else { panic!("Not a number") };
    assert_eq!(value.to_owned(), 5.0);
}

#[test]
fn test_divide_with_more_arguments() {
    let result = eval_with_data(
        "DIVIDE($foo, $bar)",
        &HashMap::from([
            ("$foo", &DalaValue::Num(10.0)),
            ("$bar", &DalaValue::Num(2.0)),
            ("$baz", &DalaValue::Num(4.0)),
        ]),
    );
    assert_eq!(result.len(), 1);

    let DalaValue::Num(value) = result[0].as_ref().unwrap() else { panic!("Not a number") };
    assert_eq!(value.to_owned(), 5.0);
}

#[test]
fn test_divide_with_data_for_single_arg() {
    let result = eval_with_data(
        "DIVIDE($foo)",
        &HashMap::from([("$foo", &DalaValue::Num(10.0))]),
    );
    assert_eq!(result.len(), 1);

    let DalaError::ParseError(err) = result[0].as_ref().unwrap_err() else { panic!("Not a parse error") };
    assert!(err.message.len() > 0);
}
