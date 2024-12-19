
use crate::tokenize::tokenize;
use crate::parse::parse;

pub fn eval(input: &str) {
    let tokens = tokenize(input);
    parse(tokens);
}