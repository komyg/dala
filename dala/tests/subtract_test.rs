use dala::{eval, DalaError, DalaValue};

#[test]
fn test_subtract() {
    let result = eval("SUBTRACT(1, 2)");
    assert_eq!(result.len(), 1);

    let DalaValue::Num(value) = result[0].as_ref().unwrap() else { panic!("Not a number") };
    assert_eq!(value.to_owned(), -1.0);
}

#[test]
fn test_subtract_with_single_arg() {
    let result = eval("SUBTRACT(1)");
    assert_eq!(result.len(), 1);

    let DalaError::ParseError(err) = result[0].as_ref().unwrap_err() else { panic!("Not a parse error") };
    assert!(err.message.len() > 0);
}

#[test]
fn test_subtract_runtime_error_with_str_arg() {
    let result = eval("SUBTRACT(1, UPPER(\"2\"))");
    assert_eq!(result.len(), 1);

    let DalaError::RuntimeError(err) = result[0].as_ref().unwrap_err() else { panic!("Not a runtime error") };
    assert_eq!(err.message, "Cannot subtract strings, found: 2");
}
