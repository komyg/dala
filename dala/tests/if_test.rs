use std::collections::HashMap;

use dala::{eval, eval_with_data, DalaValue};

#[test]
fn test_if_true() {
    let result = eval("IF(TRUE, \"Hello\", \"World\")");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value.to_owned(), "Hello");
}

#[test]
fn test_if_false() {
    let result = eval("IF(FALSE, \"Hello\", \"World\")");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value.to_owned(), "World");
}

#[test]
fn test_eq_returns_true() {
    let result = eval("IF(EQ(1, 1), \"Hello\", \"World\")");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value.to_owned(), "Hello");
}

#[test]
fn test_eq_returns_false() {
    let result = eval("IF(EQ(1, 2), \"Hello\", \"World\")");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value.to_owned(), "World");
}

#[test]
fn test_neq_returns_true() {
    let result = eval("IF(NEQ(1, 1), \"Hello\", \"World\")");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value.to_owned(), "World");
}

#[test]
fn test_neq_returns_false() {
    let result = eval("IF(NEQ(1, 2), \"Hello\", \"World\")");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value.to_owned(), "Hello");
}

#[test]
fn test_with_data() {
    let result = eval_with_data(
        "IF(EQ($arg1, $arg2), $foo, $bar)",
        &HashMap::from([
            ("$arg1", &DalaValue::Num(1.0)),
            ("$arg2", &DalaValue::Num(1.0)),
            ("$foo", &DalaValue::Str("foo".to_string())),
            ("$bar", &DalaValue::Str("bar".to_string())),
        ]),
    );
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value.to_owned(), "foo");
}

#[test]
fn test_with_mixed_data() {
    let result = eval_with_data(
        "IF(EQ($arg1, 1.0), \"foo\", $bar)",
        &HashMap::from([
            ("$arg1", &DalaValue::Num(1.0)),
            ("$bar", &DalaValue::Str("bar".to_string())),
        ]),
    );
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value.to_owned(), "foo");
}
