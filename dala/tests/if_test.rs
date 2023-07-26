use dala::{eval, DalaValue};

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
fn test_eq_true() {
    let result = eval("IF(EQ(1, 1), \"Hello\", \"World\")");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value.to_owned(), "Hello");
}

#[test]
fn test_false_true() {
    let result = eval("IF(EQ(1, 2), \"Hello\", \"World\")");
    assert_eq!(result.len(), 1);

    let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
    assert_eq!(value.to_owned(), "World");
}
