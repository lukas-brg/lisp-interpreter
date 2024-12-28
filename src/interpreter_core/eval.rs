use itertools::Itertools;

use crate::ast::AstNode;
use crate::ast::AstNodeValue;
use crate::errors::{EvalError, RuntimeError};
use crate::operatortype::Operator;
use crate::parse::parse;
use crate::tokenize::tokenize;
use crate::value::Value;

fn eval_plus(node: &AstNode) -> Result<Value, RuntimeError> {
    let mut value = Value::Int(0);
    for child in node.children() {
        value += eval_tree(child)?;
    }
    Ok(value)
}

fn eval_minus(node: &AstNode) -> Result<Value, RuntimeError> {
    let mut value = eval_tree(node.children().get(0).unwrap())?;
    if node.children().len() == 1 {
        value.negate();
        return Ok(value);
    }
    for child in node.children().iter().skip(1) {
        value -= eval_tree(child)?;
    }
    Ok(value)
}

fn eval_mul(node: &AstNode) -> Result<Value, RuntimeError> {
    let mut value = Value::Int(1);
    for child in node.children() {
        value *= eval_tree(child)?;
    }
    Ok(value)
}

fn eval_modulo(node: &AstNode) -> Result<Value, RuntimeError> {
    let mut value = eval_tree(node.children().get(0).unwrap())?;
    for child in node.children().iter().skip(1) {
        value %= eval_tree(child)?;
    }
    Ok(value)
}

fn eval_div(node: &AstNode) -> Result<Value, RuntimeError> {
    let mut value = eval_tree(node.children().get(0).unwrap())?;
    for child in node.children().iter().skip(1) {
        value /= eval_tree(child)?;
    }
    Ok(value)
}

fn eval_intdiv(node: &AstNode) -> Result<Value, RuntimeError> {
    let mut value = eval_tree(node.children().get(0).unwrap())?;
    for child in node.children().iter().skip(1) {
        value.int_div_assign(eval_tree(child)?);
    }
    Ok(value)
}

fn eval_pow(node: &AstNode) -> Result<Value, RuntimeError> {
    let mut value = eval_tree(node.children().get(0).unwrap())?;
    for child in node.children().iter().skip(1) {
        value.pow_assign(eval_tree(child)?);
    }
    Ok(value)
}

fn eval_eq(node: &AstNode) -> Result<Value, RuntimeError> {
    let value = eval_tree(node.children().get(0).unwrap())?;
    for child in node.children().iter().skip(1) {
        if eval_tree(child)? != value {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_lt(node: &AstNode) -> Result<Value, RuntimeError> {
    for (a, b) in node.children().iter().tuple_windows() {
        let (left, right) = (eval_tree(a)?, eval_tree(b)?);
        if left.compare_to(&right)? >= 0 {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_leq(node: &AstNode) -> Result<Value, RuntimeError> {
    for (a, b) in node.children().iter().tuple_windows() {
        let (left, right) = (eval_tree(a)?, eval_tree(b)?);
        if left.compare_to(&right)? > 0 {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_gt(node: &AstNode) -> Result<Value, RuntimeError> {
    for (a, b) in node.children().iter().tuple_windows() {
        let (left, right) = (eval_tree(a)?, eval_tree(b)?);
        if left.compare_to(&right)? <= 0 {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_geq(node: &AstNode) -> Result<Value, RuntimeError> {
    for (a, b) in node.children().iter().tuple_windows() {
        let (left, right) = (eval_tree(a)?, eval_tree(b)?);
        if left.compare_to(&right)? < 0 {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_neq(node: &AstNode) -> Result<Value, RuntimeError> {
    let mut vals: Vec<Value> = Vec::new();
    for child in node.children() {
        let val = eval_tree(child)?;
        vals.push(val);
    }

    for (a, b) in vals.iter().tuple_combinations() {
        if a.compare_to(b)? == 0 {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_operator(node: &AstNode, op: &Operator) -> Result<Value, RuntimeError> {
    match *op {
        Operator::Plus => eval_plus(node),
        Operator::Minus => eval_minus(node),
        Operator::Mul => eval_mul(node),
        Operator::Modulo => eval_modulo(node),
        Operator::Div => eval_div(node),
        Operator::IntDiv => eval_intdiv(node),
        Operator::Power => eval_pow(node),
        Operator::Eq => eval_eq(node),
        Operator::Lt => eval_lt(node),
        Operator::Leq => eval_leq(node),
        Operator::Gt => eval_gt(node),
        Operator::Geq => eval_geq(node),
        Operator::Neq => eval_neq(node),
        //_ => unimplemented!("Operator not implemented"),
    }
}

fn eval_tree(node: &AstNode) -> Result<Value, RuntimeError> {
    match &node.value {
        AstNodeValue::Operator(op) => eval_operator(node, op),
        AstNodeValue::Literal(v) => Ok(v.clone()),
        _ => {
            unimplemented!("Not implemented\n{}", node);
        }
    }
}

pub fn eval(input: &str) -> Result<Value, EvalError> {
    let tokens = match tokenize(input) {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(EvalError::Tokenizing(e));
        }
    };
    // println!("\nTokenize result:\n{:?}", tokens);

    let root = match parse(tokens) {
        Ok(root) => root,
        Err(e) => {
            return Err(EvalError::Parsing(e));
        }
    };
    println!("\nParse result:\n{}", root);
    let result = match eval_tree(root.children().get(0).unwrap()) {
        Ok(v) => v,
        Err(e) => return Err(EvalError::Runtime(e)),
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operators() {
        let res = eval("(1)").unwrap();
        assert_eq!(res, Value::Int(1));

        let res = eval("(-1)").unwrap();
        assert_eq!(res, Value::Int(-1));

        let res = eval("(- 1)").unwrap();
        assert_eq!(res, Value::Int(-1));

        let res = eval("(+ 1 0)").unwrap();
        assert_eq!(res, Value::Int(1));

        let res = eval("(+ 1 1)").unwrap();
        assert_eq!(res, Value::Int(2));

        let res = eval("(+ 1 -2)").unwrap();
        assert_eq!(res, Value::Int(-1));

        let res = eval("(+ 2.5 2.5)").unwrap();
        assert_eq!(res, Value::Int(5));
        let res = eval("(+ 2.5 2.6)").unwrap();
        assert_eq!(res, Value::Float(5.1));

        let res = eval("(/ 4 2)").unwrap();
        assert_eq!(res, Value::Int(2));
        let res = eval("(/ 5 2)").unwrap();
        assert_eq!(res, Value::Float(2.5));
        let res = eval("(/ 0 2)").unwrap();
        assert_eq!(res, Value::Int(0));
        let res = eval("(/ 5 -2)").unwrap();
        assert_eq!(res, Value::Float(-2.5));
        let res = eval("(/ -5 -2)").unwrap();
        assert_eq!(res, Value::Float(2.5));
        let res = eval("(// 5 2)").unwrap();
        assert_eq!(res, Value::Int(2));
        let res = eval("(% 5 2)").unwrap();
        assert_eq!(res, Value::Int(1));

        let res = eval("(% -2 24)").unwrap();
        assert_eq!(res, Value::Int(22));

        let res = eval("(* 2 3)").unwrap();
        assert_eq!(res, Value::Int(6));
        let res = eval("(* 2 2.5)").unwrap();
        assert_eq!(res, Value::Int(5));
        let res = eval("(* 2 2.1)").unwrap();
        assert_eq!(res, Value::Float(4.2));

        let res = eval("(^ 2 3)").unwrap();
        assert_eq!(res, Value::Int(8));
        let res = eval("(^ 2 -1)").unwrap();
        assert_eq!(res, Value::Float(0.5));
        let res = eval("(^ 16 (/ 1 2))").unwrap();
        assert_eq!(res, Value::Int(4));

        let res = eval("(+ (* 2 (- 10 3)) (/ (+ 15 5) (- 8 4)))").unwrap();
        assert_eq!(res, Value::Int(19));

        let res = eval("(= 5 5 5 5)").unwrap();
        assert_eq!(res, Value::Boolean(true));
        let res = eval("(= 5 5 1 5)").unwrap();
        assert_eq!(res, Value::Boolean(false));
        let res = eval("(!= 5 5 1 5)").unwrap();
        assert_eq!(res, Value::Boolean(false));
        let res = eval("(!= 1 2 3 4 1)").unwrap();
        assert_eq!(res, Value::Boolean(false));
        let res = eval("(!= 1 2 3 4 5)").unwrap();
        assert_eq!(res, Value::Boolean(true));

        let res = eval("(< 1 2 3 4 5)").unwrap();
        assert_eq!(res, Value::Boolean(true));
        let res = eval("(< 1 2 3 3 5)").unwrap();
        assert_eq!(res, Value::Boolean(false));
        let res = eval("(<= 1 2 3 3 5)").unwrap();
        assert_eq!(res, Value::Boolean(true));
        let res = eval("(<= 1 2 3 3 5 -99)").unwrap();
        assert_eq!(res, Value::Boolean(false));
        let res = eval("(< -5 -3 0 2 6)").unwrap();
        assert_eq!(res, Value::Boolean(true));

        let res = eval("(> 1 2 3 3 5)").unwrap();
        assert_eq!(res, Value::Boolean(false));
        let res = eval("(> 1 0 -1 -5)").unwrap();
        assert_eq!(res, Value::Boolean(true));
        let res = eval("(> 1 0 0 -5)").unwrap();
        assert_eq!(res, Value::Boolean(false));
        let res = eval("(>= 1 0 0 -5)").unwrap();
        assert_eq!(res, Value::Boolean(true));
    }
}
