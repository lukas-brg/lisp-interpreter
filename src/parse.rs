use crate::ast::{AstNode, AstNodeType, AstNodeValue, AstTree};
use crate::token::{Token, TokenContent, TokenType};
use crate::token::TokenContext;
use crate::token::TokenType::{LPAREN, NUMBER, OPERATOR, RPAREN};
use crate::operatortype::Operator;
use crate::operatortype::Operator::{MUL, DIV, MINUS, PLUS};

struct ParserState {
    tokens: Vec<Token>,
    tree: AstTree,
    current_token_idx: usize,
}


fn subtree(parser: &mut ParserState, parent: &mut AstNode) {
    if let Some(token) = parser.advance() {
        if let Some(TokenContent::Operator(operator)) = token.content.clone() {

            let mut operator_node = AstNode::new(
                AstNodeType::OPERATOR,
                Some(AstNodeValue::Operator(operator))
            );
            
            _parse(parser, &mut operator_node);
            parent.add_child(Box::new(operator_node));
        } else {
            panic!("Expected an operator after '(', found {:?}", token.token_type);
        }
        
    } else {
        panic!("Unexpected end of input after '('");
    }
}


fn number(token: &Token, parent: &mut AstNode) {
    let content = token.content.clone().unwrap();
    
    let ast_value  = match content {
        TokenContent::Float(v) => AstNodeValue::Float(v),
        TokenContent::Int(v) =>  AstNodeValue::Int(v),
        _ => unreachable!()
    };

    let node =
        AstNode::new(
        AstNodeType::LITERAL, 
        Some(ast_value),
        
    );
    parent.add_child(Box::new(node));
}


fn _parse(parser: &mut ParserState, parent: &mut AstNode) {
    
    while let Some(token) = parser.advance() {
        match token.token_type {
            RPAREN => return,
            LPAREN => subtree(parser, parent),
            NUMBER => number(token, parent),
            _ => {}
        };
    }
}

pub fn parse(tokens: Vec<Token>) -> Box<AstNode> {

    let mut parser = ParserState::new(tokens);
    let mut root = AstNode::new(AstNodeType::ROOT, None);

    _parse(&mut parser, &mut root);
    let tree = AstTree::new();
    Box::new(root)
}


impl ParserState {
    
    pub fn new(tokens: Vec<Token>) -> ParserState {
        
        ParserState{
            tokens,
            tree: AstTree::new(),
            current_token_idx: 0,
        }
    }
    
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current_token_idx+1)
    }
    
    pub fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current_token_idx);
        self.current_token_idx = std::cmp::min(self.tokens.len(), self.current_token_idx + 1);
        token
    }
}
