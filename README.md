# Dala Formula Language

The Dala Formula Language is a light weight language, that is heavily inspired by Excel and Google Sheets formulas, for example: `SUM(1, 2, 3, 4, 5)` will yield `15`.

It is currently a work in progress.

## The Rust API

You can use the `dala::eval` To evaluate a Dala from expression inside a Rust program. For example:

```rs
use dala::{eval, DalaValue};

let result = eval("CONCAT(\"Hello\", \" \", \"World\")");

let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
assert_eq!(value, "Hello World");
```

### The `eval` function

The `eval` function takes a string slice and returns a `Vec<Result<DalaValue, DalaError>>`, with the result of each expression that was evaluated.

The `DalaValue` enum represents the possible values that can be returned from a Dala expression: a `Str`, a `Num` or a `Boolean`.

The `DalaError` enum represents the possible errors that can occur during the evaluation:

- `BuildError`: returned when there is an processing the Dala Expression, before its evaluation.
- `RuntimeError`: returned when there is an error during the evaluation of the Dala Expression.
- `ParseError`: returned when there is an error parsing the Dala Expression.

## The Repl

You can try out the language on our Repl, by using this command: `cargo run -p repl`.

Once it is running, just type in your Dala Expression and press enter to evaluate it, for example:

```sh
> CONCAT("Hello", " ", "World")
"Hello World"
```

## The Language

### Primitives

These primitive values are used as arguments and as return values from functions.

| Primitive | Description                                      | Syntax               |
| --------- | ------------------------------------------------ | -------------------- |
| `Str`     | A string of characters enclosed by double quotes | `"Hello World"`      |
| `Num`     | A number                                         | `1`, `2.5`, `3.1415` |
| `Boolean` | A boolean value                                  | `TRUE`, `FALSE`      |

### Functions

Each function can take primitive or other functions as arguments.

| Function   | Syntax                                    | Example                                          | Description                                                                   |
| ---------- | ----------------------------------------- | ------------------------------------------------ | ----------------------------------------------------------------------------- |
| `CONCAT`   | `CONCAT(a, b, c, ...) -> Str`             | `CONCAT("Hello", " ", "World") -> "Hello World"` | Concatenates all the arguments together into a single string                  |
| `DIVIDE`   | `DIVIDE(a, b) -> Num`                     | `DIVIDE(10, 2) -> 5`                             | Divides the first argument by the second argument                             |
| `IF`       | `IF(condition, if_true, if_false) -> Lit` | `IF(TRUE, "Hello", "World") -> "World"`          | If the condition is true, evaluates `if_true`, otherwise evaluates `if_false` |
| `MULTIPLY` | `MULTIPLY(a, b) -> Num`                   | `MULTIPLY(5, 2) -> 10`                           | Multiplies the arguments                                                      |
| `SUBTRACT` | `SUBTRACT(a, b) -> Num`                   | `SUBTRACT(1, 2) -> -1`                           | Subtracts the second argument from the first                                  |
| `SUM`      | `SUM(a, b, c, ...) -> Num`                | `SUM(1, 2, 3, 4, 5) -> 15`                       | Sums all the arguments together                                               |
| `UPPER`    | `UPPER(a) -> Str`                         | `UPPER("Hello World") -> "HELLO WORLD"`          | Converts the string to upper case                                             |
