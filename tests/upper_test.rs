use dala::{eval_dala, DalaValue};

#[test]
fn test_str() {
    let result = eval_dala("UPPER(\"abc\")");
    assert_eq!(result.len(), 1);

    result.iter().for_each(|r| {
        assert_eq!(r.is_ok(), true);
        let DalaValue::Str(value) = r.as_ref().unwrap() else { panic!("Not a string") };
        assert_eq!(value, "ABC");
    });
}
