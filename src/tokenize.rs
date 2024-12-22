use core::num;
use std::str::CharIndices;

use crate::errors::TokenizingError;
use crate::operatortype::Operator::{Div, Minus, Mul, Plus};
use crate::token::TokenContext;
use crate::token::TokenType::{LparenToken, NumberToken, OperatorToken, RparenToken};
use crate::token::{Token, TokenContent, TokenType};

fn parse_number<I>(
    input: I,
    first_char: char,
    context: &TokenContext,
) -> Result<TokenContent, TokenizingError>
where
    I: IntoIterator<Item = (usize, char)>,
{
    let mut float = false;
    let mut num_str = String::from(first_char);

    for (i, c) in input {
        if !c.is_numeric() && c != '.' {
            break;
        }
        if c == '.' {
            if float {
                return Err(TokenizingError::new(
                    context.line_number,
                    context.column_number,
                    i,
                    "Invalid number: multiple decimal points found",
                ));
            }
            float = true;
        }
        num_str.push(c);
    }

    let content;
    if float {
        let parsed_float: f64 = num_str.parse().expect("Failed to parse float");
        content = TokenContent::Float(parsed_float);
    } else {
        let parsed_int: i64 = num_str.parse::<i64>().unwrap();
        content = TokenContent::Int(parsed_int);
    }

    Ok(content)
}

fn parse_string<I>(input: I, context: &TokenContext) -> Result<TokenContent, TokenizingError>
where
    I: IntoIterator<Item = (usize, char)>,
{
    let mut str = String::new();
    let mut string_closed = false;
    let mut last_index = 0;
    for (i, c) in input {
        if c == '"' {
            string_closed = true;
            break;
        }
        str.push(c);
        last_index = i;
    }
    if !string_closed {
        return Err(TokenizingError::new(
            context.line_number,
            context.column_number,
            last_index,
            "Unclosed String",
        ));
    }

    let content = TokenContent::String(str);
    Ok(content)
}

pub fn tokenize_line(
    input_str: &str,
    tokens: &mut Vec<Token>,
    line_num: usize,
) -> Result<(), TokenizingError> {
    let mut input = input_str.char_indices();

    while let Some((index, c)) = input.next() {
        if c.is_whitespace() {
            continue;
        }
        let context = TokenContext {
            line_number: line_num,
            column_number: index,
        };
        if c.is_numeric() {
            let content = parse_number(input.by_ref(), c, &context)?;
            tokens.push(Token::new(NumberToken, context, Some(content)));
            continue;
        }

        let token = match c {
            '(' => Token::new(LparenToken, context, None),
            ')' => Token::new(RparenToken, context, None),
            '+' => Token::new(
                TokenType::OperatorToken,
                context,
                Some(TokenContent::Operator(Plus)),
            ),
            '*' => Token::new(
                TokenType::OperatorToken,
                context,
                Some(TokenContent::Operator(Mul)),
            ),
            '"' => {
                let content = Some(parse_string(input.by_ref(), &context)?);
                Token::new(TokenType::OperatorToken, context, content)
            }
            _ => {
                return Err(TokenizingError::new(
                    line_num,
                    index,
                    index,
                    std::format!("Unrecognized Token '{}'", c).as_str(),
                ));
            }
        };
        tokens.push(token);
    }

    Ok(())
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizingError> {
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line) in input.lines().enumerate() {
        tokenize_line(line, &mut tokens, line_number)?
    }

    Ok(tokens)
}
