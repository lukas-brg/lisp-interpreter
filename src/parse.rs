

use crate::ast::{AstNode, AstTree, AstNodeType};
use crate::token::{Token, TokenContent, TokenType};
use crate::token::TokenContext;
use crate::token::TokenType::{LPAREN, NUMBER, PLUS, RPAREN};

struct ParserState {
    tokens: Vec<Token>,
    tree: AstTree,
    current_token_idx: usize,
    current_token: Option<Token>,
}


fn _parse(parser: &mut ParserState, parent: &mut AstNode) {
    
    
    while let Some(token) = parser.advance() {
        match token.token_type {
            LPAREN => {
                _parse(parser, parent);
            },
            RPAREN => {
                return
            }
            _ => {}
        };
    }
}

pub fn parse(tokens: Vec<Token>) {

    let mut parser = ParserState::new(tokens);
    let mut root = AstNode::new(AstNodeType::ROOT, None);

    _parse(&mut parser, &mut root);

}


impl ParserState {
    
    pub fn new(tokens: Vec<Token>) -> ParserState {
        
        let token = tokens.get(0).cloned();
        return ParserState{
            tokens,
            tree: AstTree::new(),
            current_token_idx: 0,
            current_token: token,
        };
    }
    
    pub fn peek(self) -> Option<Token> {
        let token = self.tokens.get(self.current_token_idx).cloned();
        return token;
    }
    
    pub fn advance(&mut self) -> Option<Token> {
        self.current_token_idx = std::cmp::min(self.tokens.len(), self.current_token_idx + 1);
        let token = self.tokens.get(self.current_token_idx).cloned();
        self.current_token = token.clone();
        return token;
    }
}