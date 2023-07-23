use dala::{eval_dala, DalaValue};

#[test]
fn test_concat_two_strings() {
    let result = eval_dala("CONCAT(\"hello\", \"world\")");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value, "helloworld");
}

#[test]
fn test_concat_multiple_literals() {
    let result = eval_dala("CONCAT(\"hello\", \" \", 123, \" \", TRUE)");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value, "hello 123 TRUE");
}

#[test]
fn test_with_nested_function() {
    let result = eval_dala("CONCAT(\"hello\", \" \", UPPER(\"world\"))");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value, "hello WORLD");
}
