use crate::ast::{AstNode, AstNodeType, AstNodeValue, AstTree, Operator};
use crate::token::{Token, TokenContent, TokenType};
use crate::token::TokenContext;
use crate::token::TokenType::{LPAREN, NUMBER, PLUS, RPAREN};

struct ParserState {
    tokens: Vec<Token>,
    tree: AstTree,
    current_token_idx: usize,
}


fn subtree(parser: &mut ParserState, parent: &mut AstNode) {
    if let Some(operator_token) = parser.advance() {
        match operator_token.token_type {
            PLUS | TokenType::MINUS | TokenType::DIV | TokenType::MULT => {
                
                let mut operator_node = AstNode::new(
                    AstNodeType::OPERATOR,
                    Some(AstNodeValue::Operator(
                        match operator_token.token_type {
                            TokenType::PLUS => Operator::PLUS,
                            TokenType::MINUS => Operator::MINUS,
                            TokenType::MULT => Operator::MUL,
                            TokenType::DIV => Operator::DIV,
                            _ => unreachable!(), 
                        }
                    )),
                );

                _parse(parser, &mut operator_node);
                parent.add_child(Box::new(operator_node));
            },
            _ => {
                panic!("Expected an operator after '(', found {:?}", operator_token.token_type);
            },
        };
    } else {
        panic!("Unexpected end of input after '('");
    }
}


fn number(parent: &mut AstNode, token: &Token) {
    let content = token.content.clone().unwrap();
    let ast_value;
    match content {
        TokenContent::Float(v) => {
            ast_value = AstNodeValue::Float(v);
        },
        TokenContent::Int(v) => {
            ast_value = AstNodeValue::Int(v);
        }
        _ => {unreachable!()}

    };

    let node =
        AstNode::new(
        AstNodeType::LITERAL, 
        Some(ast_value)
        
    );
    parent.add_child(Box::new(node));
}


fn _parse(parser: &mut ParserState, parent: &mut AstNode) {
    
    while let Some(token) = parser.advance() {
        match token.token_type {
            RPAREN => {
                return
            },
            LPAREN => {
                subtree(parser, parent);
            },
            NUMBER => {
                number(parent, token);
            },
          
            _ => {}
        };
    }
}

pub fn parse(tokens: Vec<Token>) {

    let mut parser = ParserState::new(tokens);
    let mut root = AstNode::new(AstNodeType::ROOT, None);

    _parse(&mut parser, &mut root);
    print!("\n\n{}", root);
}


impl ParserState {
    
    pub fn new(tokens: Vec<Token>) -> ParserState {
        
        return ParserState{
            tokens,
            tree: AstTree::new(),
            current_token_idx: 0,
        };
    }
    
    pub fn peek(&self) -> Option<&Token> {
        let token = self.tokens.get(self.current_token_idx+1);
        return token;
    }
    
    pub fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current_token_idx);
        self.current_token_idx = std::cmp::min(self.tokens.len(), self.current_token_idx + 1);
        return token;
    }
}
