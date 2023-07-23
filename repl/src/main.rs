use dala::eval;
use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        let result = eval(input);

        result
            .into_iter()
            .for_each(|dala_result| match dala_result {
                Ok(value) => println!("{}", value),
                Err(err) => println!("{}", err),
            });
    }
}
