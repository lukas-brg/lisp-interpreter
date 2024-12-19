
use std::iter::Cloned;

use crate::ast::{AstNode, AstTree};
use crate::token::{Token, TokenContent};
use crate::token::TokenContext;
use crate::token::TokenType::{LPAREN, NUMBER, PLUS, RPAREN};

struct ParserState {
    tokens: Vec<Token>,
    tree: AstTree,
    current_token_idx: usize,
    current_token: Option<Token>,
}

pub fn parse(tokens: Vec<Token>) {

    let parser = ParserState::new(tokens);


}


impl ParserState {
    
    pub fn new(tokens: Vec<Token>) -> ParserState {
        
        let token = tokens.get(0).cloned();
        return ParserState{
            tokens: tokens,
            tree: AstTree::new(),
            current_token_idx: 0,
            current_token: token,
        };
    }
    
    pub fn peek(self) -> Option<Token> {
        let token = self.tokens.get(self.current_token_idx).cloned();
        return token;
    }
    
    pub fn advance(&mut self) {
        self.current_token_idx = std::cmp::min(self.tokens.len(), self.current_token_idx + 1);
        let token = self.tokens.get(self.current_token_idx).cloned();
        self.current_token = token;
    }
}