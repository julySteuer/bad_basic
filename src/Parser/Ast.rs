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
    pub args: Vec<Box<AstNode>>,
    pub ast_type: AstTypes,
    pub right: Option<Box<AstNode>>,
    pub left: Option<Box<AstNode>>
}

// Also implement exec method
impl AstNode {
    pub fn new(value: String, ast_type: AstTypes) -> AstNode {
        AstNode { value, args: Vec::new(),ast_type,right: None, left: None }
    }

    pub fn set_args(&mut self, args: Vec<Box<AstNode>>) {
        self.args = args;
    }

    pub fn add_args(&mut self, arg: AstNode) {
        self.args.push(Box::new(arg));
    }

    pub fn set_right(&mut self, node: AstNode) {
        self.right = Some(Box::new(node))
    }

    pub fn set_left(&mut self, node: AstNode) {
        self.left = Some(Box::new(node))
    }
}