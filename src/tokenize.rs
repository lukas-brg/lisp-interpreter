
use crate::token::{Token, TokenContent, TokenType};
use crate::token::TokenContext;
use crate::token::TokenType::{LPAREN, NUMBER, OPERATOR, RPAREN};
use crate::operatortype::Operator::{MUL, DIV, MINUS, PLUS};
use crate::errors::TokenizerError;



fn parse_number(input: &str, start: usize, context: &TokenContext) -> Result<(usize, TokenContent), TokenizerError> {

    let mut chars = input[start..].chars();
    let mut float = false;
    let mut end = start;
    
    for (i, c) in chars.enumerate() {
        if !c.is_numeric() && c != '.' {
            break;
        }
        if c == '.' {
            if float {
                return Err(
                    TokenizerError::new(
                        context.line_number, 
                        start, 
                        i, 
                        "Invalid number: multiple decimal points found"
                    )
                );
            }
            float = true;
        }
        end += 1;
    }

    let num_str = &input[start..end];
       
    let content;
    if float {
        let parsed_float: f64 = num_str.parse().expect("Failed to parse float");
        content = TokenContent::Float(parsed_float);
    } else {
        let parsed_int: i64 = num_str.parse::<i64>().unwrap();
        content = TokenContent::Int(parsed_int);
    }

    return Ok((end, content));
}



pub fn tokenize_line(input: &str, tokens: &mut Vec<Token>, line_num: usize) -> Result<(), TokenizerError> {
    let mut index = 0;
    while index < input.len() {

        let c = input.as_bytes()[index] as char;
        
        if c.is_whitespace() {
            index += 1;
            continue;
        }

        let context = TokenContext{
            line_number: line_num,
            column_number: index,
        };


        if c.is_numeric() {
            
            match parse_number(input, index, &context) {
                Ok((i, content)) => {
                    index = i;
                    tokens.push(Token::new(NUMBER, context, Some(content)));
                    continue;
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        
        match c {
            '(' => {
                let t = Token::new(LPAREN, context, None);
                tokens.push(t);
            },
            ')' => {
                let t = Token::new(RPAREN, context, None);
                tokens.push(t);
            },
            '+' => {
                let t = Token::new(TokenType::OPERATOR, context, Some(TokenContent::Operator(PLUS)));
                tokens.push(t);
            },
            '*' => {
                let t = Token::new(TokenType::OPERATOR, context, Some(TokenContent::Operator(MUL)));
                tokens.push(t);
            },
            
            '"' => {
                
            }
            
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
        index += 1;
    }
    return Ok(());
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line) in input.lines().enumerate() {
        match tokenize_line(line, &mut tokens, line_number) {
            Ok(()) => {},
            Err(e) => {
                return Err(e);
            }
        }
    }

    return Ok(tokens);

}