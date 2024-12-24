use crate::ast::{AstNode, AstNodeType, AstNodeValue};
use crate::errors::ParsingError;
use crate::operatortype::Operator;
use crate::operatortype::Operator::{Div, Minus, Mul, Plus};
use crate::token::{Token, TokenContent, TokenContext, TokenType};

struct ParserState {
    tokens: Vec<Token>,
    current_token_idx: usize,
}

fn subtree(parser: &mut ParserState, parent: &mut AstNode) -> Result<(), ParsingError> {
    if let Some(token) = parser.advance() {
        if let Some(TokenContent::Operator(operator)) = token.content.clone() {
            let mut operator_node = AstNode::new(
                AstNodeType::Operator,
                Some(AstNodeValue::Operator(operator)),
            );
            _parse(parser, &mut operator_node)?;
            parent.add_child(operator_node);
        } else {
            let err = ParsingError::new(
                Some(token.clone()),
                format!(
                    "Expected an operator after '(', found {:?}",
                    token.token_type
                )
                .as_str(),
            );
            return Err(err);
        }
    } else {
        let err = ParsingError::new(None, "Expected token after '(' found EOF");
        return Err(err);
    }

    Ok(())
}

fn number(token: &Token, parent: &mut AstNode) {
    let content = token.content.clone().unwrap();

    let ast_value = match content {
        TokenContent::Float(v) => AstNodeValue::Float(v),
        TokenContent::Int(v) => AstNodeValue::Int(v),
        _ => unreachable!(),
    };

    let node = AstNode::new(AstNodeType::Literal, Some(ast_value));
    parent.add_child(node);
}

fn _parse(parser: &mut ParserState, parent: &mut AstNode) -> Result<(), ParsingError> {
    while let Some(token) = parser.advance() {
        match token.token_type {
            TokenType::Rparen => return Ok(()),
            TokenType::Lparen => subtree(parser, parent)?,
            TokenType::Number => number(token, parent),
            _ => {}
        };
    }

    Ok(())
}

pub fn parse(tokens: Vec<Token>) -> Result<AstNode, ParsingError> {
    let mut parser = ParserState::new(tokens);
    let mut root = AstNode::new(AstNodeType::Root, None);
    _parse(&mut parser, &mut root)?;
    Ok(root)
}

impl ParserState {
    pub fn new(tokens: Vec<Token>) -> ParserState {
        ParserState {
            tokens,
            current_token_idx: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current_token_idx + 1)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current_token_idx);
        self.current_token_idx = std::cmp::min(self.tokens.len(), self.current_token_idx + 1);
        token
    }

    pub fn has_next(&self) -> bool {
        self.current_token_idx < self.tokens.len() - 1
    }
}
