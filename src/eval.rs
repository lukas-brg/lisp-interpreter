use crate::ast;
use crate::ast::AstNode;
use crate::ast::AstNodeType;
use crate::ast::AstNodeValue;
use crate::operatortype::Operator;
use crate::parse::parse;
use crate::tokenize::tokenize;
use itertools::Itertools;
use std::fmt::write;
use std::ops::Deref;
use std::ops::{Add, Div, Mul, Sub};
use std::path::Display;

#[derive(Debug, Clone)]
enum Value {
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    None,
}

impl Value {
    fn promote_to_float(self) -> Self {
        match self {
            Value::Int(i) => Value::Float(i as f64),
            other => other,
        }
    }

    fn is_numeric(&self) -> bool {
        matches!(self, Value::Int(_) | Value::Float(_))
    }

    fn from_ast_node_value(value: &AstNodeValue) -> Self {
        return match value {
            AstNodeValue::Int(v) => Value::Int(*v),
            AstNodeValue::Float(v) => Value::Float(*v),
            AstNodeValue::String(v) => Value::String(v.clone()),
            _ => unreachable!(),
        };
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l + r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l + r),
            (Value::Int(l), Value::Float(r)) => Value::Float(l as f64 + r),
            (Value::Float(l), Value::Int(r)) => Value::Float(l + r as f64),
            (Value::String(l), Value::String(r)) => Value::String(l + &r),
            _ => Value::None,
        }
    }
}

impl std::ops::AddAssign for Value {
    fn add_assign(&mut self, rhs: Value) {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => *l += r,
            (Value::Float(l), Value::Float(r)) => *l += r,
            (Value::Int(l), Value::Float(r)) => *l += r as i64,
            (Value::Float(l), Value::Int(r)) => *l += r as f64,
            _ => {
                unreachable!();
            }
        }
    }
}

impl std::ops::MulAssign for Value {
    fn mul_assign(&mut self, rhs: Value) {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => *l *= r,
            (Value::Float(l), Value::Float(r)) => *l *= r,
            (Value::Int(l), Value::Float(r)) => *l *= r as i64,
            (Value::Float(l), Value::Int(r)) => *l *= r as f64,
            _ => {
                unreachable!();
            }
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l * r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l * r),
            (Value::Int(l), Value::Float(r)) => Value::Float(l as f64 * r),
            (Value::Float(l), Value::Int(r)) => Value::Float(l * r as f64),
            _ => Value::None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(l), Value::Int(r)) => l == r,
            (Value::Float(l), Value::Float(r)) => (l - r).abs() < f64::EPSILON,
            (Value::Boolean(l), Value::Boolean(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            _ => false,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn eval_tree(node: &AstNode) -> Value {
    match node.node_type {
        AstNodeType::Operator => {
            if let AstNodeValue::Operator(op) = &node.node_value {
                match *op {
                    Operator::Plus => {
                        let mut value = Value::Int(0);
                        for child in node.children() {
                            value += eval_tree(child);
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
                    _ => unimplemented!("Operator not implemented"),
                }
            } else {
                unreachable!("Node must have operator value");
            }
        }
        AstNodeType::Literal => {
            return Value::from_ast_node_value(&node.node_value);
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
    println!("\nParse result:\n{}", root);
    let v = eval_tree(&root.children().get(0).unwrap());
    println!("{}", v);
}
