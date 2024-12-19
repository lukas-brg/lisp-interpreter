
#[derive(Debug, Clone)]
pub enum Operator {
    PLUS,
    MINUS,
    DIV,
    MUL,
    UnaryMinus,
    UnaryPlus, 
}

#[derive(Debug, Clone)]
pub enum AstNodeValue {
    String(String),
    Float(f64),
    Int(i64),
    
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
        return AstNode {
            node_type: node_type,
            node_value: node_value,
            children: Vec::new(),
        }
    }
}

impl AstTree {

    pub fn new() -> AstTree {
        let root = AstNode::new(
            AstNodeType::ROOT,
            None,
        );

        return AstTree {
            root: root
        }
    }

}

