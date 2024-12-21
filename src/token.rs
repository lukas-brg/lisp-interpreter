use crate::operatortype::Operator;


#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    LPAREN,
    RPAREN,
    OPERATOR,
    NUMBER,
    IDENTIFIER,
    STRING,
}

#[derive(Debug, Clone)]
pub struct TokenContext {
    pub line_number: usize,
    pub column_number: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub context: TokenContext,
    pub content: Option<TokenContent>, 
}

#[derive(Debug, Clone)]
pub enum TokenContent {
    Float(f64),
    Int(i64),
    String(String),
    Operator(Operator),
}

impl Token {
    pub fn new(token_type: TokenType, context: TokenContext, content: Option<TokenContent>) -> Self {
        return Token {
            token_type,
            context,
            content,
        };
    }
}