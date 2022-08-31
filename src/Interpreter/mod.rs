use core::panic;
use std::collections::HashMap;
use crate::Parser::{Ast::AstNode, Parser, Ast::AstTypes};
mod BuildInFuncs;

/*
pub enum ReturnTypes {
    STRING(String),
    INT(i32)
}
*/
// TODO: replace i32 with return types
pub struct Interpreter {
    line_map: HashMap<String, AstNode>,
    line_order: Vec<String>,
    index: usize,
    line_number: String,
    memory: HashMap<String, i32>
}
/*

TODO: Add return type enum for primitives
*/
impl Interpreter {
    pub fn new(parser: Parser) -> Interpreter {
        let map = parser.get_lines().clone();
        let mut line_order = Vec::from_iter(map.keys().cloned());
        line_order.sort();
        Interpreter { line_map: parser.get_lines().clone(), index: 0, line_order, line_number: String::new(), memory: HashMap::new() }
    }

    pub fn new_from_map(map: &HashMap<String, AstNode>) -> Interpreter {
        let mut line_order = Vec::from_iter(map.keys().cloned());
        line_order.sort();
        Interpreter { line_map: map.clone(), index: 0, line_order, line_number: String::new(), memory: HashMap::new() }
    }

    pub fn eval(&mut self, node: &AstNode) -> Option<i32> {
        match node.ast_type {
            AstTypes::OP => Some(self.eval_bin_op(node)), // eval bin node here
            AstTypes::ASSIGN => Some(self.eval_assign(node)),
            AstTypes::FUNC => self.eval_func(node),
            AstTypes::NUMBER => Some(node.value.parse().unwrap()),
            AstTypes::IDENT => Some(self.get_variable(&node.value)),
            _ => None
        }
    }

    fn eval_func(&mut self, node: &AstNode) -> Option<i32> {
        let eval = self.eval(&*node.args[0]);
        match &node.value[..] {
            "PRINT" => BuildInFuncs::PRINT(eval.unwrap().to_string()),
            _ => println!("Error: Function definition not found for {}", &node.value)
        }
        None
    }
    
    fn eval_bin_op(&mut self, node: &AstNode) -> i32 {
        let left = &node.left.clone().unwrap();
        let right = &node.right.clone().unwrap();
        match &node.value[..] {
            "+" => self.eval(left).unwrap() + self.eval(right).unwrap(),
            "-" => self.eval(left).unwrap() - self.eval(right).unwrap(),
            _ => panic!("Binary Operator not found")
        }
    }

    // TODO: Maybe macro implementation
    fn eval_assign(&mut self, node: &AstNode) -> i32 {
        let left = &node.left.clone().unwrap();
        let right = &node.right.clone().unwrap();

        let val = self.eval(right).unwrap();
        self.memory.insert(left.value.to_owned(), val);
        val
    }
    
    fn get_variable(&self, name: &String) -> i32 {
        let var = self.memory.get(name);
        if var.is_none() {
            panic!("Variable not found");
        };
        var.unwrap().clone()
    }

    pub fn run_line(&mut self) -> i32 {
        self.set_line_number_by_index();
        let current = self.get_safe_current();
        let res = self.eval(&current);
        if res.is_none() {
            return 0
        }
        res.unwrap()
    }

    pub fn run(&mut self) -> i32 { // Parses everything not line by line 
        self.set_line_number_by_index();
        let mut last: i32 = 0;
        while self.get_current().is_some() {
            last = self.run_line();
            if self.get_next().is_none() {
                break;
            };
        };
        last
    }

    pub fn add_parser_data(&mut self, parser: Parser) {
        self.line_map.extend(parser.get_lines().clone())
    }

    fn get_current(&self) -> Option<&AstNode> {
        self.line_map.get(&self.line_number)
    }

    fn get_safe_current(&self) -> AstNode {
        if self.get_current().is_none() {
            panic!("Unknown Line number");
        }
        self.get_current().unwrap().clone()
    }
    // because it can be overriden by goto command 
    fn get_next(&mut self) -> Option<&AstNode> {
        self.index = self.line_order.iter().position(|r| r == &self.line_number).unwrap();
        self.index += 1;
        if self.index >= self.line_order.len() {
            return None
        };
        self.line_number = self.line_order[self.index].clone();
        self.get_by_line_number(&self.line_order[self.index])
    }

    fn get_by_line_number(&self, line_number: &String) -> Option<&AstNode> {
        self.line_map.get(line_number)
    }

    fn get_by_index(&mut self) -> Option<&AstNode> {
        let line_number = self.line_order[self.index].clone();
        self.line_map.get(&line_number)
    }

    fn set_line_number_by_index(&mut self) {
        self.line_number = self.line_order[self.index].clone();
    }
}