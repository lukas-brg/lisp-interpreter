use std::fmt;
use crate::operatortype::Operator;



#[derive(Debug, Clone)]
pub enum AstNodeValue {
    String(String),
    Float(f64),
    Int(i64),
    Operator(Operator),
    
}


#[derive(Debug, Clone)]
pub enum AstNodeType {
    OPERATOR,
    LITERAL,
    IDENTIFIER,
    ROOT,
}


#[derive(Debug, Clone)]
pub struct AstNode {
    node_type: AstNodeType,
    node_value: Option<AstNodeValue>,
    children: Vec<Box<AstNode>>,
}

#[derive(Debug, Clone)]
pub struct AstTree {

    root: AstNode,
}

impl AstNode {
    pub fn new(node_type: AstNodeType, node_value: Option<AstNodeValue>) -> AstNode {
        AstNode {
            node_type,
            node_value,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, node: Box<AstNode>) {
        self.children.push(node);
    }

    pub fn children(&self) -> &Vec<Box<AstNode>> {
        &self.children
    }
    
}

impl AstTree {

    pub fn new() -> AstTree {
        let root = AstNode::new(
            AstNodeType::ROOT,
            None,
        );

        AstTree {
            root,
        }
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
        
        write!(f, "{}Node Type: {:?}, ", indent, self.node_type)?;
        
        if let Some(value) = &self.node_value {
            write!(f, "Value: {:?}, ", value)?;
        }

        if !self.children.is_empty() {
            write!(f, "Children:\n")?;
            for child in &self.children {
                child.fmt_with_indent(f, indent_level + 1)?;  
            }
        } else {
            write!(f, "No children")?;
        }

        write!(f, "\n")  
    }
}

impl fmt::Display for AstNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for AstNodeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNodeValue::Int(i) => write!(f, "{}", i),
            AstNodeValue::Float(fl) => write!(f, "{}", fl),
            AstNodeValue::String(s) => write!(f, "'{}'", s),
            _ => { write!(f, "Int: ")}
        }
    }
}