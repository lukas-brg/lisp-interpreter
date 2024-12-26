use crate::eval::eval;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{self, Write};

mod ast;
mod env;
mod errors;
mod eval;
mod operatortype;
mod parse;
mod token;
mod tokenize;
mod value;

fn is_complete_expression(input: &str) -> bool {
    let open = input.matches('(').count();
    let close = input.matches(')').count();
    open == close
}

fn main() {
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(".repl_history");
    let mut buffer = String::new();
    loop {
        let prompt = if buffer.is_empty() { "> " } else { ">> " };
        match rl.readline(prompt) {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed == "exit" || trimmed == "quit" {
                    break;
                }
                buffer.push_str(trimmed);
                if buffer.is_empty() {
                    continue;
                }

                if is_complete_expression(&buffer) {
                    buffer = if !(buffer.starts_with('(') && buffer.ends_with(')')) {
                        format!("({})", buffer)
                    } else {
                        buffer
                    };
                    rl.add_history_entry(&buffer);
                    eval(&buffer);
                    buffer.clear();
                } else {
                    buffer.push(' ');
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(".repl_history").unwrap_or_else(|e| {
        eprintln!("Failed to save history: {:?}", e);
    });
}
