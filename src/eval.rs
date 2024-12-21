
use crate::tokenize::tokenize;
use crate::parse::parse;
use crate::ast::AstNode;


fn eval_tree(root: Box<AstNode>) {
    for node in root.children() {
        
    }
}

pub fn eval(input: &str) {
    let tokens = tokenize(input);
    let root = parse(tokens);
    print!("\nParse result:\n{}", root);
    eval_tree(root);
}