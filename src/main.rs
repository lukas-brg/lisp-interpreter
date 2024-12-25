use crate::eval::eval;

mod ast;
mod env;
mod errors;
mod eval;
mod operatortype;
mod parse;
mod token;
mod tokenize;

fn main() {
    eval("(* 2 (+ 2 4))");
}
