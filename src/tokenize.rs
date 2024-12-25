use std::str::CharIndices;

use itertools::Itertools;

use crate::errors::TokenizingError;
use crate::operatortype::Operator::{Div, IntDiv, Minus, Mul, Plus};
use crate::token::TokenContext;
use crate::token::TokenType::{Lparen, Number, Operator, Rparen};
use crate::token::{Token, TokenContent, TokenType};

fn parse_number<I>(
    input: &mut std::iter::Peekable<I>,
    first_char: char,
    context: &TokenContext,
) -> Result<TokenContent, TokenizingError>
where
    I: std::iter::Iterator<Item = (usize, char)>,
{
    let mut float = false;
    let mut num_str = String::from(first_char);

    while let Some(&(i, c)) = input.peek() {
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
        input.next();
    }

    let content = if float {
        let parsed_float: f64 = num_str.parse().expect("Failed to parse float");
        TokenContent::Float(parsed_float)
    } else {
        let parsed_int: i64 = num_str.parse::<i64>().unwrap();
        TokenContent::Int(parsed_int)
    };

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
    let mut input = input_str.char_indices().peekable();

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
            tokens.push(Token::new(Number, context, Some(content)));
            continue;
        }

        let token = match c {
            '(' => Token::new(Lparen, context, None),
            ')' => Token::new(Rparen, context, None),
            '+' => Token::new(
                TokenType::Operator,
                context,
                Some(TokenContent::Operator(Plus)),
            ),
            '-' => {
                let mut is_unary_minus = false;
                if let Some((_, next_c)) = input.peek() {
                    if next_c.is_numeric() {
                        is_unary_minus = true;
                    }
                };

                let token = if is_unary_minus {
                    let content = parse_number(input.by_ref(), c, &context)?;
                    Token::new(TokenType::Number, context, Some(content))
                } else {
                    Token::new(
                        TokenType::Operator,
                        context,
                        Some(TokenContent::Operator(Minus)),
                    )
                };

                token
            }
            '*' => Token::new(
                TokenType::Operator,
                context,
                Some(TokenContent::Operator(Mul)),
            ),
            '/' => {
                let content = if let Some((_, '/')) = input.peek() {
                    input.next();
                    TokenContent::Operator(IntDiv)
                } else {
                    TokenContent::Operator(Div)
                };

                Token::new(TokenType::Operator, context, Some(content))
            }

            '"' => {
                let content = Some(parse_string(input.by_ref(), &context)?);
                Token::new(TokenType::String, context, content)
            }
            _ => {
                if c.is_alphabetic() {
                    let mut identifier = String::from(c);
                    while let Some(&(_, c)) = input.peek() {
                        if c.is_alphanumeric() || c == '-' {
                            identifier.push(c);
                            input.next();
                        } else {
                            break;
                        }
                    }
                    Token::new(
                        TokenType::Identifier,
                        context,
                        Some(TokenContent::String(identifier)),
                    )
                } else {
                    return Err(TokenizingError::new(
                        line_num,
                        index,
                        index,
                        std::format!("Unrecognized Token '{}'", c).as_str(),
                    ));
                }
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
