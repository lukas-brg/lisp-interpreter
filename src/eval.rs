use itertools::Itertools;

use crate::ast;
use crate::ast::AstNode;
use crate::ast::AstNodeValue;
use crate::errors::{EvalError, ParsingError, RuntimeError, TokenizingError};
use crate::operatortype::Operator;
use crate::parse::parse;
use crate::tokenize::tokenize;
use crate::value::Value;

fn eval_tree(node: &AstNode) -> Result<Value, RuntimeError> {
    match &node.value {
        AstNodeValue::Operator(op) => match *op {
            Operator::Plus => {
                let mut value = Value::Int(0);
                for child in node.children() {
                    value += eval_tree(child)?;
                }
                return Ok(value);
            }
            Operator::Minus => {
                let mut value = eval_tree(node.children().get(0).unwrap())?;
                if node.children().len() == 1 {
                    value.negate();
                    return Ok(value);
                }
                for child in node.children().iter().skip(1) {
                    value -= (eval_tree(child)?);
                }
                return Ok(value);
            }
            Operator::Mul => {
                let mut value = Value::Int(1);
                for child in node.children() {
                    value *= eval_tree(child)?;
                }
                return Ok(value);
            }
            Operator::Modulo => {
                let mut value = eval_tree(node.children().get(0).unwrap())?;
                for child in node.children().iter().skip(1) {
                    value %= eval_tree(child)?;
                }
                return Ok(value);
            }
            Operator::Div => {
                let mut value = eval_tree(node.children().get(0).unwrap())?;
                for child in node.children().iter().skip(1) {
                    value /= eval_tree(child)?;
                }
                return Ok(value);
            }
            Operator::IntDiv => {
                let mut value = eval_tree(node.children().get(0).unwrap())?;
                for child in node.children().iter().skip(1) {
                    value.int_div_assign(eval_tree(child)?);
                }
                return Ok(value);
            }
            Operator::Power => {
                let mut value = eval_tree(node.children().get(0).unwrap())?;
                for child in node.children().iter().skip(1) {
                    value.pow_assign(eval_tree(child)?);
                }
                return Ok(value);
            }
            Operator::Eq => {
                let mut value = eval_tree(node.children().get(0).unwrap())?;
                for child in node.children().iter().skip(1) {
                    if eval_tree(child)? != value {
                        return Ok(Value::Boolean(false));
                    }
                }
                return Ok(Value::Boolean(true));
            }
            Operator::Lt => {
                for (a, b) in node.children().iter().tuple_windows() {
                    let (left, right) = (eval_tree(a)?, eval_tree(b)?);
                    if left.compare_to(&right)? >= 0 {
                        return Ok(Value::Boolean(false));
                    }
                }
                return Ok(Value::Boolean(true));
            }
            Operator::Leq => {
                for (a, b) in node.children().iter().tuple_windows() {
                    let (left, right) = (eval_tree(a)?, eval_tree(b)?);
                    if left.compare_to(&right)? > 0 {
                        return Ok(Value::Boolean(false));
                    }
                }
                return Ok(Value::Boolean(true));
            }
            Operator::Gt => {
                for (a, b) in node.children().iter().tuple_windows() {
                    let (left, right) = (eval_tree(a)?, eval_tree(b)?);
                    if left.compare_to(&right)? <= 0 {
                        return Ok(Value::Boolean(false));
                    }
                }
                return Ok(Value::Boolean(true));
            }
            Operator::Geq => {
                for (a, b) in node.children().iter().tuple_windows() {
                    let (left, right) = (eval_tree(a)?, eval_tree(b)?);
                    if left.compare_to(&right)? < 0 {
                        return Ok(Value::Boolean(false));
                    }
                }
                return Ok(Value::Boolean(true));
            }
            _ => unimplemented!("Operator not implemented"),
        },
        AstNodeValue::Literal(v) => {
            return Ok(v.clone());
        }

        _ => {
            println!("Unreachable {}", node);
            unreachable!();
        }
    }
}

pub fn eval(input: &str) -> Result<(), EvalError> {
    let tokens = match tokenize(input) {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(EvalError::TokenizingError(e));
        }
    };
    // println!("\nTokenize result:\n{:?}", tokens);

    let root = match parse(tokens) {
        Ok(root) => root,
        Err(e) => {
            return Err(EvalError::ParsingError(e));
        }
    };
    // println!("\nParse result:\n{}", root);
    let v = match eval_tree(&root.children().get(0).unwrap()) {
        Ok(v) => v,
        Err(e) => return Err(EvalError::RuntimeError(e)),
    };
    println!("{}", v);
    Ok(())
}
