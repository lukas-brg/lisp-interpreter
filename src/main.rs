use crate::errors::EvalError;
use crate::eval::eval;

use rustyline::error::ReadlineError;
use rustyline::KeyPress;
use rustyline::{Cmd, CompletionType, Config, EditMode, Editor};

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

    rl.bind_sequence(
        KeyPress::ControlLeft,
        Cmd::Move(rustyline::Movement::BackwardWord(1, rustyline::Word::Emacs)),
    );
    rl.bind_sequence(
        KeyPress::ControlRight,
        Cmd::Move(rustyline::Movement::ForwardWord(
            1,
            rustyline::At::BeforeEnd,
            rustyline::Word::Emacs,
        )),
    );

    loop {
        let prompt = if buffer.is_empty() { "> " } else { ".. " };
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

                    if let Err(err) = eval(&buffer) {
                        eprintln!("Error: {}", err.message());
                    }

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
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(".repl_history").unwrap_or_else(|e| {
        eprintln!("Failed to save history: {:?}", e);
    });
}
