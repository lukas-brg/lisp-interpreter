use crate::eval::eval;

mod token;
mod tokenize;
mod eval;
mod ast;
mod parse;

fn main() {
    println!("Lisp interpreter");
    eval("(+ 1 (* 2 3))");
}
