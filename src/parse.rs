use crate::ast::{AstNode, AstNodeType, AstNodeValue, AstTree};
use crate::errors::ParsingError;
use crate::operatortype::Operator;
use crate::operatortype::Operator::{Div, Minus, Mul, Plus};
use crate::token::TokenContext;
use crate::token::TokenType::{LparenToken, NumberToken, OperatorToken, RparenToken};
use crate::token::{Token, TokenContent, TokenType};

struct ParserState {
    tokens: Vec<Token>,
    tree: AstTree,
    current_token_idx: usize,
}

fn subtree(parser: &mut ParserState, parent: &mut AstNode) -> Result<(), ParsingError> {
    if let Some(token) = parser.advance() {
        if let Some(TokenContent::Operator(operator)) = token.content.clone() {
            let mut operator_node = AstNode::new(
                AstNodeType::OperatorNode,
                Some(AstNodeValue::Operator(operator)),
            );

            _parse(parser, &mut operator_node);
            parent.add_child(Box::new(operator_node));
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

    let node = AstNode::new(AstNodeType::LiteralNode, Some(ast_value));
    parent.add_child(Box::new(node));
}

fn _parse(parser: &mut ParserState, parent: &mut AstNode) -> Result<(), ParsingError> {
    while let Some(token) = parser.advance() {
        match token.token_type {
            RparenToken => return Ok(()),
            LparenToken => subtree(parser, parent)?,
            NumberToken => number(token, parent),
            _ => {}
        };
    }

    Ok(())
}

pub fn parse(tokens: Vec<Token>) -> Result<Box<AstNode>, ParsingError> {
    let mut parser = ParserState::new(tokens);
    let mut root = AstNode::new(AstNodeType::RootNode, None);

    _parse(&mut parser, &mut root)?;
    let tree = AstTree::new();
    Ok(Box::new(root))
}

impl ParserState {
    pub fn new(tokens: Vec<Token>) -> ParserState {
        ParserState {
            tokens,
            tree: AstTree::new(),
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
}
