use crate::ast::AstNode;
use crate::parse::parse;
use crate::tokenize::tokenize;

fn eval_tree(root: Box<AstNode>) {
    for node in root.children() {}
}

pub fn eval(input: &str) {
    let tokens = match tokenize(input) {
        Ok(tokens) => tokens,
        Err(e) => {
            panic!("\n{}", e);
        }
    };

    let root = match parse(tokens) {
        Ok(root) => root,
        Err(e) => {
            panic!("\n{}", e);
        }
    };
    println!("\nParse result:\n{}", root);
    eval_tree(root);
}
