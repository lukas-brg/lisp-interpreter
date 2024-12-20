use crate::eval::eval;

mod token;
mod tokenize;
mod eval;
mod ast;
mod parse;
mod env;
mod operatortype;

fn main() {
    eval("(+ 1 (* 2 3))");
}

