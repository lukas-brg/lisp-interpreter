
use crate::token::{Token, TokenContent};
use crate::token::TokenContext;

fn parse_number(input: &str, start: usize) -> (usize, TokenContent) {

    let mut index = start + 1;

    let mut num_str = String::new();

    let mut c = input.as_bytes()[0] as char;
    num_str.push(c);
    let mut float = false;
    while index < input.len() && (c.is_numeric() || c == '.') {
        if c == '.'{
            float = true;
        }
        num_str.push(c);
        index += 1;
    }
    let content;

    if float {
        let parsed_float: f64 = num_str.parse().expect("Failed to parse float");
        content = TokenContent::Float(parsed_float);
    } else {
        let parsed_int: i32 = num_str.parse().expect("Failed to parse int");
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

        let c = input.as_bytes()[index] as char; // Get the character at the current index

        if c.is_numeric() {}


        let token = match c {
            ' ' | '\n' | '\t' => {
                // Skip whitespace (space, newline, tab)
                index += 1; // Move to the next character
                continue; // Continue to the next iteration of the loop
            },

            _ => {}
        };

    }
    index += 1;
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line) in input.lines().enumerate() {

        println!("Line {}: {}", line_number + 1, line);
        tokenize_line(line, &mut tokens, line_number)

    }
    return tokens;

}