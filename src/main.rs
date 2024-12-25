use crate::eval::eval;
use std::io::{self, Write};

mod ast;
mod env;
mod errors;
mod eval;
mod operatortype;
mod parse;
mod token;
mod tokenize;

fn main() {
    //eval("(* 2 (+ 2.5 4))");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input. Try again.");
            continue;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "exit" || trimmed == "quit" {
            println!("Goodbye!");
            break;
        }

        let processed_input = if !(trimmed.starts_with('(') && trimmed.ends_with(')')) {
            format!("({})", trimmed)
        } else {
            trimmed.to_string()
        };

        eval(&processed_input);
    }
}
