use crate::ast;
use crate::ast::AstNode;
use crate::ast::AstNodeValue;
use crate::ast::Value;
use crate::operatortype::Operator;
use crate::parse::parse;
use crate::tokenize::tokenize;
use itertools::Itertools;
use std::fmt::write;
use std::ops::Deref;
use std::ops::{Add, Div, Mul, Sub};
use std::path::Display;

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
        if let Value::Int(l) = self {
            if let Value::Int(r) = rhs {
                *l += r;
                return;
            }
        }

        let result: f64 = match (&*self, rhs) {
            (Value::Float(l), Value::Float(r)) => *l + r,
            (Value::Int(l), Value::Float(r)) => (*l as f64) + r,
            (Value::Float(l), Value::Int(r)) => *l + (r as f64),
            _ => unreachable!(),
        };

        *self = Value::Float(result);
    }
}

impl std::ops::MulAssign for Value {
    fn mul_assign(&mut self, rhs: Value) {
        if let Value::Int(l) = self {
            if let Value::Int(r) = rhs {
                *l *= r;
                return;
            }
        }

        let result: f64 = match (&*self, rhs) {
            (Value::Float(l), Value::Float(r)) => *l * r,
            (Value::Int(l), Value::Float(r)) => (*l as f64) * r,
            (Value::Float(l), Value::Int(r)) => *l * (r as f64),
            _ => unreachable!(),
        };

        *self = Value::Float(result);
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
    match &node.value {
        AstNodeValue::Operator(op) => match *op {
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
