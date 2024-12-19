
use crate::token::{Token, TokenContent};
use crate::token::TokenContext;
use crate::token::TokenType::{LPAREN, NUMBER, PLUS, RPAREN};

fn parse_number(input: &str, start: usize) -> (usize, TokenContent) {

    let mut index = start + 1;

    let mut num_str = String::new();

    let mut c = input.as_bytes()[start] as char;
    num_str.push(c);
    let mut float = false;
    c = input.as_bytes()[index] as char;
    while index < input.len() && (c.is_numeric() || c == '.') {
        c = input.as_bytes()[index] as char;
        if c == '.'{
            float = true;
        }
        num_str.push(c);
        index += 1;
    }
    let content;

    if float {
        let parsed_float: f64 = num_str.trim().parse().expect("Failed to parse float");
        content = TokenContent::Float(parsed_float);
    } else {
        println!(" {}", num_str);
        let parsed_int: i32 = num_str.trim().parse::<i32>().unwrap();
        content = TokenContent::Int(parsed_int);
    }

    return (index, content);
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
                let t = Token::new(PLUS, context, None);
                tokens.push(t);
            },

            _ => {}
        };

        index += 1;
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line) in input.lines().enumerate() {

        println!("Line {}: {}", line_number + 1, line);
        tokenize_line(line, &mut tokens, line_number)
    }

    println!("Tokens: {:?}", tokens);
    return tokens;

}