#[derive(Debug, Clone, PartialEq)]
pub enum AstTypes {
    NUMBER,
    STRING,
    IDENT,
    ASSIGN,
    OP,
    EXPR,
    FUNC
}

#[derive(Debug, Clone)]
pub struct AstNode {
    pub value: String,
    pub ast_type: AstTypes,
    pub right: Option<Box<AstNode>>,
    pub left: Option<Box<AstNode>>
}

// Also implement exec method
impl AstNode {
    pub fn new(value: String, ast_type: AstTypes) -> AstNode {
        AstNode { value, ast_type,right: None, left: None }
    }

    pub fn set_right(&mut self, node: AstNode) {
        self.right = Some(Box::new(node))
    }

    pub fn set_left(&mut self, node: AstNode) {
        self.left = Some(Box::new(node))
    }
}