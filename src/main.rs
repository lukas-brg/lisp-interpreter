use crate::eval::eval;

mod token;
mod tokenize;
mod eval;

fn main() {
    println!("Lisp interpreter");
    eval("(+ 1 2)")
}
