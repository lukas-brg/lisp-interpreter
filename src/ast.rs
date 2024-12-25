use crate::operatortype::Operator;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
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
}

#[derive(Debug, Clone)]
pub enum AstNodeValue {
    Operator(Operator),
    Literal(Value),
    Identifier(String),
    Root,
}

#[derive(Debug, Clone)]
pub struct AstNode {
    pub value: AstNodeValue,
    children: Vec<AstNode>,
}

impl AstNode {
    pub fn new(value: AstNodeValue) -> AstNode {
        AstNode {
            value,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, node: AstNode) {
        self.children.push(node);
    }

    pub fn children(&self) -> &Vec<AstNode> {
        &self.children
    }
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

impl AstNode {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent_level: usize) -> fmt::Result {
        let indent = "  ".repeat(indent_level);

        write!(f, "{}Node Type: {:?}, ", indent, self.value)?;

        if !self.children.is_empty() {
            writeln!(f, "Children:")?;
            for child in &self.children {
                child.fmt_with_indent(f, indent_level + 1)?;
            }
        } else {
            write!(f, "No children")?;
        }

        writeln!(f)
    }
}

impl fmt::Display for AstNodeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
