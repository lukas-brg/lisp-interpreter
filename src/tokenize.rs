
use core::num;
use std::str::CharIndices;

use crate::token::{Token, TokenContent, TokenType};
use crate::token::TokenContext;
use crate::token::TokenType::{LPAREN, NUMBER, OPERATOR, RPAREN};
use crate::operatortype::Operator::{MUL, DIV, MINUS, PLUS};
use crate::errors::TokenizerError;



fn parse_number<I>(input: I, first_char: char, context: &TokenContext) -> Result<TokenContent, TokenizerError> 
where I: IntoIterator<Item = (usize, char)>
{
    let mut float = false;
    let mut num_str = String::from(first_char);

    for (i, c) in input {
        if !c.is_numeric() && c != '.' {
            break;
        }
        if c == '.' {
            if float {
                return Err(
                    TokenizerError::new(
                        context.line_number, 
                        context.column_number, 
                        i, 
                        "Invalid number: multiple decimal points found"
                    )
                );
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



pub fn tokenize_line(input_str: &str, tokens: &mut Vec<Token>, line_num: usize) -> Result<(), TokenizerError> {
    
    let mut input = input_str.char_indices();
    
    while let Some((index, c)) = input.next() {

        if c.is_whitespace() {
            continue;
        }

        let context = TokenContext{
            line_number: line_num,
            column_number: index,
        };


        if c.is_numeric() {
            let content = parse_number(input.by_ref(), c, &context)?;
            tokens.push(Token::new(NUMBER, context, Some(content)));
            continue;
        }
        
        let token = match c {
            '(' => Token::new(LPAREN, context, None),
            ')' => Token::new(RPAREN, context, None),
            '+' => Token::new(TokenType::OPERATOR, context, Some(TokenContent::Operator(PLUS))),
            '*' => Token::new(TokenType::OPERATOR, context, Some(TokenContent::Operator(MUL))),
            '"' => Token::new(TokenType::OPERATOR, context, Some(TokenContent::Operator(MUL))),
            _ => {
                return Err(
                    TokenizerError::new(
                        line_num, 
                        index, 
                        index, 
                        std::format!("Unrecognized Token '{}'", c).as_str()
                    )
                );
            }
        };
        tokens.push(token);
    }
    
    Ok(())
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line) in input.lines().enumerate() {
        tokenize_line(line, &mut tokens, line_number)?
    }

    Ok(tokens)
}