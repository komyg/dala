use dala::{eval, DalaValue};

fn main() {
    let result = eval("SUM(1, 2, 3, 4, -5)");
    let DalaValue::Num(value) = result[0].as_ref().unwrap() else { panic!("Not a number") };
    println!("{}", value);
}
