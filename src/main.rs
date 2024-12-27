use crate::eval::eval;

mod ast;
mod env;
mod errors;
mod eval;
mod operatortype;
mod parse;
mod repl;
mod token;
mod tokenize;
mod value;

fn main() {
    repl::run_repl();
}
