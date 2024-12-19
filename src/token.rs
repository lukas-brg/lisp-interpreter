#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    LPAREN,
    RPAREN,
    PLUS,
    MINUS,
    MULT,
    DIV,
    NUMBER,
    IDENTIFIER,
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
    Int(i32),
    String(String),
}

impl Token {
    pub fn new(token_type: TokenType, context: TokenContext, content: Option<TokenContent>) -> Self {
        Token {
            token_type,
            context,
            content,
        }
    }
}