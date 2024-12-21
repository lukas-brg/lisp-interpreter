use std::fmt;

#[derive(Debug)]
pub struct TokenizerError {
    pub line_number: usize,
    pub token_index: usize,
    pub error_index: usize,
    pub message: String,
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error on line {}, column {}, error index: {}: {}", // ToDo Improve error messages
            self.line_number, self.token_index, self.error_index, self.message
        )
    }
}

impl TokenizerError {
    pub fn new(line_number: usize, token_index: usize, error_index: usize, message: &str) -> Self {
        TokenizerError {
            line_number,
            token_index: token_index,
            error_index,
            message: message.to_string(),
        }
    }
}