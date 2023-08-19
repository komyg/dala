use std::collections::HashMap;

use dala::{eval, eval_with_data, DalaValue};

#[test]
fn test_concat_two_strings() {
    let result = eval("CONCAT(\"hello\", \"world\")");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value, "helloworld");
}

#[test]
fn test_concat_multiple_literals() {
    let result = eval("CONCAT(\"hello\", \" \", 123, \" \", TRUE)");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value, "hello 123 TRUE");
}

#[test]
fn test_with_nested_function() {
    let result = eval("CONCAT(\"hello\", \" \", UPPER(\"world\"))");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value, "hello WORLD");
}

#[test]
fn test_with_data() {
    let result = eval_with_data(
        "CONCAT($var1, $2)",
        &HashMap::from([
            ("$var1", &DalaValue::Str("hello".to_string())),
            ("$2", &DalaValue::Str("world".to_string())),
        ]),
    );
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value, "helloworld");
}
