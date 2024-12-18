use crate::tokenize::tokenize;

mod token;
mod tokenize;

fn main() {
    println!("Lisp interpreter");
    tokenize("(+ 1 2)");
}
