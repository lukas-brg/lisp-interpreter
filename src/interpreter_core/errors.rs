use std::fmt;

use crate::token;

#[derive(Debug)]
pub struct TokenizingError {
    pub line_number: usize,
    pub token_index: usize,
    pub error_index: usize,
    pub message: String,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(msg: String) -> RuntimeError {
        RuntimeError { message: msg }
    }
}
#[derive(Debug)]
pub enum EvalError {
    Tokenizing(TokenizingError),
    Parsing(ParsingError),
    Runtime(RuntimeError),
}
impl EvalError {
    pub fn message(&self) -> &str {
        match self {
            EvalError::Tokenizing(err) => &err.message,
            EvalError::Parsing(err) => &err.message,
            EvalError::Runtime(err) => &err.message,
        }
    }
}

impl fmt::Display for TokenizingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error on line {}, column {}, error index: {}: {}", // ToDo Improve error messages
            self.line_number, self.token_index, self.error_index, self.message
        )
    }
}

impl TokenizingError {
    pub fn new(line_number: usize, token_index: usize, error_index: usize, message: &str) -> Self {
        TokenizingError {
            line_number,
            token_index,
            error_index,
            message: message.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ParsingError {
    pub token: Option<token::Token>,
    pub message: String,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Parsing Error: {}", // ToDo Improve error messages
            self.message
        )
    }
}

impl ParsingError {
    pub fn new(token: Option<token::Token>, message: &str) -> Self {
        ParsingError {
            token,
            message: message.to_string(),
        }
    }
}
