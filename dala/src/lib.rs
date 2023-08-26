use core::fmt;
use pest::Span;
use std::collections::HashMap;

mod interpreter;
use interpreter::expr::eval_visitor::EvalVisitor;
use interpreter::{ast::create_ast, parser::parse_dala};

/// The result of a successful evaluation of a `DalaExpression`.
/// It can be either a `String`, a `f64` or a `bool`.
#[derive(Debug, Clone)]
pub enum DalaValue {
    Str(String),
    Num(f64),
    Boolean(bool),
}

impl fmt::Display for DalaValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DalaValue::Str(value) => write!(f, "\"{}\"", value),
            DalaValue::Num(value) => write!(f, "{}", value),
            DalaValue::Boolean(value) => write!(f, "{}", value.to_string().to_uppercase()),
        }
    }
}

/// Contains the position of a `DalaExpression` in the source code.
#[derive(Debug, Clone)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn new(pair: Span) -> Self {
        Self {
            start: pair.start(),
            end: pair.end(),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.start, self.end)
    }
}

/// The result of an unsuccessful evaluation of a `DalaExpression`.
#[derive(Debug, Clone)]
pub enum DalaError {
    BuildError(BuildError),
    RuntimeError(RuntimeError),
    ParseError(ParseError),
}

impl fmt::Display for DalaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DalaError::BuildError(err) => write!(f, "{}", err),
            DalaError::RuntimeError(err) => write!(f, "{}", err),
            DalaError::ParseError(err) => write!(f, "{}", err),
        }
    }
}

/// An error that occurs during the evaluation of a `DalaExpression`.
#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub pos: Position,
    pub message: String,
}

impl RuntimeError {
    pub fn new(pos: Position, message: String) -> Self {
        Self { pos, message }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime error: {}, at: {}", self.message, self.pos)
    }
}

/// An error that occurs when processing the a `DalaExpression`, before its evaluation.
#[derive(Debug, Clone)]
pub struct BuildError {
    pub pos: Position,
    pub message: String,
}

impl BuildError {
    pub fn new(pos: Position, message: String) -> Self {
        Self { pos, message }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Build error: {}, at: {}", self.message, self.pos)
    }
}

/// An error that occurs when parsing a `DalaExpression`.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

/// Evaluates a `DalaExpression` and returns a `DalaValue` if the evaluation is successful or a `DalaError` if it is not.
///
/// # Examples
///
/// ```
/// use dala::{eval, DalaValue};
///
/// let result = eval("CONCAT(\"Hello\", \" \", \"World\")");
/// let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
/// assert_eq!(value, "Hello World");
/// ```
pub fn eval(str: &str) -> Vec<Result<DalaValue, DalaError>> {
    let parsed = parse_dala(str);
    if parsed.is_err() {
        return vec![Err(parsed.unwrap_err())];
    }

    create_ast(parsed.unwrap(), &HashMap::new())
        .into_iter()
        .map(|expr| expr.and_then(|expr| expr.eval()))
        .collect()
}

/// Evaluates a `DalaExpression` with the provied dataset and returns a `DalaValue` if the evaluation is successful or a `DalaError` if it is not.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use dala::{eval_with_data, DalaValue};
///
/// let result = eval_with_data(
///     "CONCAT($var1, $2)",
///     &HashMap::from([
///         ("$var1", &DalaValue::Str("hello".to_string())),
///         ("$2", &DalaValue::Str("world".to_string())),
///     ]),
/// );
/// let DalaValue::Str(value) = result[0].as_ref().unwrap() else { panic!("Not a string") };
/// assert_eq!(value, "helloworld");
/// ```
pub fn eval_with_data(
    str: &str,
    data: &HashMap<&str, &DalaValue>,
) -> Vec<Result<DalaValue, DalaError>> {
    let parsed = parse_dala(str);
    if parsed.is_err() {
        return vec![Err(parsed.unwrap_err())];
    }

    create_ast(parsed.unwrap(), data)
        .into_iter()
        .map(|expr| expr.and_then(|expr| expr.eval()))
        .collect()
}
