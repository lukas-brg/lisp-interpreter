use crate::ast;
use crate::ast::AstNode;
use crate::ast::AstNodeValue;
use crate::operatortype::Operator;
use crate::parse::parse;
use crate::tokenize::tokenize;
use crate::value::Value;

fn eval_tree(node: &AstNode) -> Value {
    match &node.value {
        AstNodeValue::Operator(op) => match *op {
            Operator::Plus => {
                let mut value = Value::Int(0);
                for child in node.children() {
                    value += eval_tree(child);
                }
                return value;
            }
            Operator::Minus => {
                let mut value = eval_tree(node.children().get(0).unwrap());
                for child in node.children().iter().skip(1) {
                    value -= eval_tree(child);
                }
                return value;
            }
            Operator::Mul => {
                let mut value = Value::Int(1);
                for child in node.children() {
                    value *= eval_tree(child);
                }
                return value;
            }
            Operator::Div => {
                let mut value = eval_tree(node.children().get(0).unwrap());
                for child in node.children().iter().skip(1) {
                    value /= eval_tree(child);
                }
                return value;
            }
            Operator::IntDiv => {
                let mut value = eval_tree(node.children().get(0).unwrap());
                for child in node.children().iter().skip(1) {
                    value.int_div_assign(eval_tree(child));
                }
                return value;
            }
            _ => unimplemented!("Operator not implemented"),
        },
        AstNodeValue::Literal(v) => {
            return v.clone();
        }

        _ => {
            println!("Unreachable {}", node);
            unreachable!();
        }
    }
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
    // println!("\nParse result:\n{}", root);
    let v = eval_tree(&root.children().get(0).unwrap());
    println!("{}", v);
}
