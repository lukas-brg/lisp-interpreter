use itertools::Itertools;

use crate::ast::AstNode;
use crate::ast::AstNodeValue;
use crate::env::Environment;
use crate::errors::{EvalError, RuntimeError};
use crate::operatortype::Operator;
use crate::parse::parse;
use crate::tokenize::tokenize;
use crate::value::Value;

fn eval_plus(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    let mut value = Value::Int(0);
    for child in node.children() {
        value += eval_tree(child, env)?;
    }
    Ok(value)
}

fn eval_minus(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    let mut value = eval_tree(node.children().get(0).unwrap(), env)?;
    if node.children().len() == 1 {
        value.negate();
        return Ok(value);
    }
    for child in node.children().iter().skip(1) {
        value -= eval_tree(child, env)?;
    }
    Ok(value)
}

fn eval_mul(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    let mut value = Value::Int(1);
    for child in node.children() {
        value *= eval_tree(child, env)?;
    }
    Ok(value)
}

fn eval_modulo(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    let mut value = eval_tree(node.children().get(0).unwrap(), env)?;
    for child in node.children().iter().skip(1) {
        value %= eval_tree(child, env)?;
    }
    Ok(value)
}

fn eval_div(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    let mut value = eval_tree(node.children().get(0).unwrap(), env)?;
    for child in node.children().iter().skip(1) {
        value /= eval_tree(child, env)?;
    }
    Ok(value)
}

fn eval_intdiv(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    let mut value = eval_tree(node.children().get(0).unwrap(), env)?;
    for child in node.children().iter().skip(1) {
        value.int_div_assign(eval_tree(child, env)?);
    }
    Ok(value)
}

fn eval_pow(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    let mut value = eval_tree(node.children().get(0).unwrap(), env)?;
    for child in node.children().iter().skip(1) {
        value.pow_assign(eval_tree(child, env)?);
    }
    Ok(value)
}

fn eval_eq(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    let value = eval_tree(node.children().get(0).unwrap(), env)?;
    for child in node.children().iter().skip(1) {
        if eval_tree(child, env)? != value {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_lt(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    for (a, b) in node.children().iter().tuple_windows() {
        let (left, right) = (eval_tree(a, env)?, eval_tree(b, env)?);
        if left.compare_to(&right)? >= 0 {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_leq(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    for (a, b) in node.children().iter().tuple_windows() {
        let (left, right) = (eval_tree(a, env)?, eval_tree(b, env)?);
        if left.compare_to(&right)? > 0 {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_gt(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    for (a, b) in node.children().iter().tuple_windows() {
        let (left, right) = (eval_tree(a, env)?, eval_tree(b, env)?);
        if left.compare_to(&right)? <= 0 {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_geq(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    for (a, b) in node.children().iter().tuple_windows() {
        let (left, right) = (eval_tree(a, env)?, eval_tree(b, env)?);
        if left.compare_to(&right)? < 0 {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_neq(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    let mut vals: Vec<Value> = Vec::new();
    for child in node.children() {
        let val = eval_tree(child, env)?;
        vals.push(val);
    }

    for (a, b) in vals.iter().tuple_combinations() {
        if a.compare_to(b)? == 0 {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}

fn eval_operator(
    node: &AstNode,
    op: &Operator,
    env: &mut Environment,
) -> Result<Value, RuntimeError> {
    match *op {
        Operator::Plus => eval_plus(node, env),
        Operator::Minus => eval_minus(node, env),
        Operator::Mul => eval_mul(node, env),
        Operator::Modulo => eval_modulo(node, env),
        Operator::Div => eval_div(node, env),
        Operator::IntDiv => eval_intdiv(node, env),
        Operator::Power => eval_pow(node, env),
        Operator::Eq => eval_eq(node, env),
        Operator::Lt => eval_lt(node, env),
        Operator::Leq => eval_leq(node, env),
        Operator::Gt => eval_gt(node, env),
        Operator::Geq => eval_geq(node, env),
        Operator::Neq => eval_neq(node, env),
        //_ => unimplemented!("Operator not implemented"),
    }
}

fn eval_identifier(
    node: &AstNode,
    identifier: &String,
    env: &mut Environment,
) -> Result<AstNode, RuntimeError> {
    let mut new_node = node.clone();
    if let Some(val) = env.get_var(identifier) {
        let mut flattened_subtree = AstNode::new(AstNodeValue::Values);
        flattened_subtree.add_child(AstNode::new(AstNodeValue::Literal(val.clone())));
        flattened_subtree.append_children(node.children().clone());
        println!("{}", flattened_subtree);
        return Ok(flattened_subtree);
    }

    match identifier.as_str() {
        "defvar" => {}
        _ => unimplemented!(),
    }
    Ok(new_node)
}

fn eval_tree(node: &AstNode, env: &mut Environment) -> Result<Value, RuntimeError> {
    if let AstNodeValue::Identifier(v) = &node.value {
        let mut n = eval_identifier(node, v, env)?;
        return eval_tree(&n, env);
    } else {
        match &node.value {
            AstNodeValue::Operator(op) => eval_operator(node, op, env),
            AstNodeValue::Literal(v) => Ok(v.clone()),

            _ => {
                unimplemented!("Not implemented\n{}", node);
            }
        }
    }
}

pub fn eval_with_env(input: &str, env: &mut Environment) -> Result<Value, EvalError> {
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
    let result = match eval_tree(&mut root.children().get(0).unwrap(), env) {
        Ok(v) => v,
        Err(e) => return Err(EvalError::Runtime(e)),
    };
    println!("\nParse result:\n{}", root);
    Ok(result)
}

pub fn eval(input: &str) -> Result<Value, EvalError> {
    let mut env = Environment::new();
    env.set_var(&"pi".to_string(), Value::Float(3.14));
    eval_with_env(input, &mut env)
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
