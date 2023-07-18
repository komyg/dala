use dala::{eval_dala, DalaValue};

#[test]
fn test_str() {
    let result = eval_dala("UPPER(\"abc\")");
    assert_eq!(result.len(), 1);

    result.iter().for_each(|r| {
        let DalaValue::Str(value) = r.as_ref().unwrap() else { panic!("Not a string") };
        assert_eq!(value, "ABC");
    });
}

#[test]
#[ignore]
fn test_int() {
    let result = eval_dala("UPPER(123)");
    assert_eq!(result.len(), 1);

    result.iter().for_each(|r| {
        let DalaValue::Str(value) = r.as_ref().unwrap() else { panic!("Not a string") };
        assert_eq!(value, "123");
    });
}
