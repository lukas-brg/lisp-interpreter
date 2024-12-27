use crate::ast::{AstNode, AstNodeValue};
use crate::errors::ParsingError;
use crate::token::{Token, TokenContent, TokenType};
use crate::value::Value;

struct ParserState {
    tokens: Vec<Token>,
    next_token_idx: usize,
}
impl ParserState {
    pub fn new(tokens: Vec<Token>) -> ParserState {
        ParserState {
            tokens,
            next_token_idx: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.next_token_idx)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.next_token_idx);
        self.next_token_idx = std::cmp::min(self.tokens.len(), self.next_token_idx + 1);
        token
    }

    pub fn has_next(&self) -> bool {
        self.next_token_idx < self.tokens.len() - 1
    }
}

fn number(token: &Token, parent: &mut AstNode) {
    let content = token.content.clone().unwrap();

    let ast_value = match content {
        TokenContent::Float(v) => AstNodeValue::Literal(Value::Float(v)),
        TokenContent::Int(v) => AstNodeValue::Literal(Value::Int(v)),
        _ => unreachable!(),
    };

    let node = AstNode::new(ast_value);
    parent.add_child(node);
}

fn identifier(token: &Token, parent: &mut AstNode) {
    let content = token.content.clone().unwrap();

    if let TokenContent::String(v) = content {
        let ast_value = AstNodeValue::Identifier(v);
        let node = AstNode::new(ast_value);
        parent.add_child(node);
    } else {
        unreachable!();
    }
}

fn string(token: &Token, parent: &mut AstNode) {
    let content = token.content.clone().unwrap();

    if let TokenContent::String(v) = content {
        let ast_value = AstNodeValue::Literal(Value::String(v));
        let node = AstNode::new(ast_value);
        parent.add_child(node);
    } else {
        unreachable!();
    }
}

fn expression(parser: &mut ParserState, parent: &mut AstNode) -> Result<(), ParsingError> {
    if let Some(token) = parser.peek() {
        if let Some(TokenContent::Operator(operator)) = token.content.clone() {
            let mut operator_node = AstNode::new(AstNodeValue::Operator(operator));
            let _ = parser.advance();
            _parse(parser, &mut operator_node)?;
            parent.add_child(operator_node);
        } else {
            _parse(parser, parent)?;
        }
    } else {
        let err = ParsingError::new(None, "Expected token after '(' found EOF");
        return Err(err);
    }
    Ok(())
}

fn _parse(parser: &mut ParserState, parent: &mut AstNode) -> Result<(), ParsingError> {
    while let Some(token) = parser.advance() {
        match token.token_type {
            TokenType::Rparen => return Ok(()),
            TokenType::Lparen => expression(parser, parent)?,
            TokenType::Number => number(token, parent),
            TokenType::Identifier => identifier(token, parent),
            TokenType::String => string(token, parent),
            TokenType::Quote => {}
            _ => {}
        };
    }

    Ok(())
}

pub fn parse(tokens: Vec<Token>) -> Result<AstNode, ParsingError> {
    let mut parser = ParserState::new(tokens);
    let mut root = AstNode::new(AstNodeValue::Root);
    _parse(&mut parser, &mut root)?;
    Ok(root)
}
