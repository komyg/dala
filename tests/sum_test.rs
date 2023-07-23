use dala::{eval_dala, DalaError, DalaValue};

#[test]
fn test_sum() {
    let result = eval_dala("SUM(1, 2, 3, 4, -5)");
    assert_eq!(result.len(), 1);

    let DalaValue::Num(value) = result[0].as_ref().unwrap() else { panic!("Not a number") };
    assert_eq!(value.to_owned(), 5.0);
}

#[test]
fn test_sum_with_single_arg() {
    let result = eval_dala("SUM(1)");
    assert_eq!(result.len(), 1);

    let DalaValue::Num(value) = result[0].as_ref().unwrap() else { panic!("Not a number") };
    assert_eq!(value.to_owned(), 1.0);
}

#[test]
fn test_sum_runtime_error_with_str_arg() {
    let result = eval_dala("SUM(1, UPPER(\"2\"))");
    assert_eq!(result.len(), 1);

    let DalaError::RuntimeError(err) = result[0].as_ref().unwrap_err() else { panic!("Not a number") };
    assert_eq!(err.message, "Cannot sum strings, found: 2");
}
