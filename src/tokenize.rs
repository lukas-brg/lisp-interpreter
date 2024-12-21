
use crate::token::{Token, TokenContent, TokenType};
use crate::token::TokenContext;
use crate::token::TokenType::{LPAREN, NUMBER, OPERATOR, RPAREN};
use crate::operatortype::Operator::{MUL, DIV, MINUS, PLUS};



fn parse_number(input: &str, start: usize) -> (usize, TokenContent) {

    let mut chars = input[start..].chars();
    let mut float = false;
    let mut end = start;
    
    for (i, c) in chars.enumerate() {
        if !c.is_numeric() && c != '.' {
            break;
        }
        if c == '.'{
            if float {
                panic!("Invalid number: multiple decimal points found");
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

    return (end, content);
}


pub fn tokenize_line(input: &str, tokens: &mut Vec<Token>, line_num: usize) {
    let mut index = 0;
    while index < input.len() {

        let context = TokenContext{
            line_number: line_num,
            column_number: index,
        };

        let c = input.as_bytes()[index] as char;

        if c.is_numeric() {
            let (i, content) = parse_number(input, index);
            index = i;
            tokens.push(Token::new(NUMBER, context, Some(content)));
            continue;
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

            _ => {}
        };

        index += 1;
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line) in input.lines().enumerate() {
        tokenize_line(line, &mut tokens, line_number)
    }

    return tokens;

}