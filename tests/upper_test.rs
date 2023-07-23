use dala::{eval, DalaValue};

#[test]
fn test_str() {
    let result = eval("UPPER(\"abc\")");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value, "ABC");
}

#[test]
fn test_int() {
    let result = eval("UPPER(123)");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value, "123");
}

#[test]
fn test_bool() {
    let result = eval("UPPER(TRUE)");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value, "TRUE");
}

#[test]
fn test_expr() {
    let result = eval("UPPER(CONCAT(\"abc\", \"def\"))");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value, "ABCDEF");
}
