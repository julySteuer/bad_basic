use core::panic;
use std::collections::HashMap;

use crate::Tokenizer::{Token, Tokenizer, TokenTypes};

use self::Ast::{AstNode, AstTypes};

pub mod Ast;

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    line_map: HashMap<String, AstNode>,
    index: usize
}
/*
struct <Name> <dt1>/<dt2>/<dtn>
*/
impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Parser {
        Parser { tokens: tokenizer.get_tokens().clone().to_vec(), index: 0, line_map: HashMap::new() }
    }

    fn parse_curr(&mut self) -> AstNode{
        // Check all line parse options
        // Primitive types at the bottom
        if let Some(i) = self.parse_assign() {
            return i;
        }
        if let Some(i) = self.parse_term() {
            return i;
        }
        if let Some(i) = self.parse_func() {
            return i;
        }
        if let Some(i) = self.parse_number() {
            return i;
        }
        if let Some(i) = self.parse_ident() {
            return i;
        }

        panic!("No Parse Option Found");
    }

    fn get_line_dist(&self) -> Vec<usize>{
        let mut res: Vec<usize> = Vec::new();
        let mut index: usize = 0;
        let mut curr_count: usize = 0;
        while index <= self.tokens.len()-1 {
            if self.tokens[index].token_type == TokenTypes::LINEBREAK {
                curr_count = 0;
                res.push(curr_count);
            }
            curr_count += 1;
            index += 1;
        }
        if res.len() == 0 {
            res.push(index);
        }
        res
    }

    pub fn parse(&mut self) {
        let counts = self.get_line_dist();
        let mut i: usize = 0;
        while !self.eos() { // maybe try here with other break condition
            self.parse_line();
            self.index += counts[i];
            i += 1;
        }
    }

    pub fn parse_line(&mut self) {
        let line_number = self.get_line_number();
        if line_number.is_none() {
            panic!("No line Number Provided at {}", self.index);
        }
        let line_number = line_number.unwrap();
        let parsed_value = self.parse_curr();
        self.line_map.insert(line_number, parsed_value);
    }

    pub fn get_number(&self) -> Option<String>{
        if self.get_current().token_type == TokenTypes::NUMBER {
            return Some(self.get_current().value);
        }
        return None;
    }

    fn get_line_number(&mut self) -> Option<String> {
        let line_number = self.get_number();
        if !self.eat_whitespace() {
            panic!("Correct Linenumber not provided");
        };
        line_number
    }

    fn eat_whitespace(&mut self) -> bool {
        self.eat(TokenTypes::WHITESPACE)
    }

    fn eat_paren_start(&mut self) -> bool {
        self.eat(TokenTypes::LPAREN)
    }

    fn eat_paren_end(&mut self) -> bool { // Maybe generalize
        self.eat(TokenTypes::RPAREN)
    }

    fn eat_comma(&mut self) -> bool {
        self.eat(TokenTypes::COMMA)
    }

    fn eat(&mut self, token_type: TokenTypes) -> bool{
        self.index += 1;
        if self.get_current().token_type == token_type {
            self.index += 1;
            return true;
        }
        return false;
    }

    fn get_keyword_type(&mut self) -> Option<AstTypes> {
        match &self.get_current().value[..] {
            "LET" => Some(AstTypes::ASSIGN),
            _ => None // Is not keyword so is string or if idet pos than error
        }
    }

    fn parse_ident(&self) -> Option<AstNode> {
        if self.get_current().token_type == TokenTypes::STRING {
            return Some(AstNode::new(self.get_current().value, AstTypes::IDENT));
        }
        return None
    }

    fn parse_number(&self) -> Option<AstNode>{
        if self.get_current().token_type == TokenTypes::NUMBER {
            return Some(AstNode::new(self.get_current().value, Ast::AstTypes::NUMBER));
        }
        return None
    }

    fn parse_op(&self) -> Option<String> {
        if self.get_current().token_type == TokenTypes::OP {
            return Some(self.get_current().value);
        }
        return None
    }

    fn get_current(&self) -> Token{
        self.tokens[self.index].clone()
    }

    fn eos(&self) -> bool {
        self.index >= self.tokens.len()
    }

    pub fn get_lines(&self) -> &HashMap<String, AstNode> {
        &self.line_map
    }

    // Make to real Recursive decent parser with recursion
    fn parse_term(&mut self) -> Option<AstNode> {
        let mut left: Option<AstNode> = None;
        let mut op: Option<String> = None;
        let mut right: Option<AstNode> = None;
        if let Some(left_node) = self.parse_number() {
            left = Some(left_node);
        } else if let Some(left_node) = self.parse_ident() {
            left = Some(left_node)
        }
        else {return None;}
        self.index += 1;
        if self.eos() {self.index -= 1;return None;};
        if let Some(i)= self.parse_op() {
            op = Some(i);
        }
        else {self.index -= 1;return None;}
        self.index += 1;
        if self.eos() {return None;};

        if let Some(node) = self.parse_term() {
            right = Some(node);
        } else if let Some(number_node) = self.parse_number() {
            right = Some(number_node);
        } else if let Some(right_node) = self.parse_ident() {
            right = Some(right_node);
        }

        let mut ast_node = AstNode::new(op.unwrap(), Ast::AstTypes::OP);
        ast_node.set_left(left.unwrap());
        ast_node.set_right(right.unwrap());
        Some(ast_node)
    }

    // Add to memeory
    fn parse_assign(&mut self) -> Option<AstNode> {
        let mut left: Option<AstNode> = None;
        let mut right: Option<AstNode> = None;
        let keyword = self.get_keyword_type();
        if keyword.is_none() {
            return None
        }
        if keyword.unwrap() != AstTypes::ASSIGN {
            return None
        }
        self.index += 1;
        self.eat_whitespace();
        left = self.parse_ident();
        self.index += 1;
        if self.get_current().token_type != TokenTypes::ASSIGN {
            return None
        }
        self.index += 1;
        let is_next_term = self.parse_term();
        if is_next_term.is_some() {
            right = is_next_term
        };
        let is_next_num_literal = self.parse_number();
        if is_next_num_literal.is_some() {
            right = is_next_num_literal
        } else {
            panic!("Not assignable to unknown");
        };
        let mut node = AstNode::new("=".to_string(), AstTypes::ASSIGN);
        node.set_right(right.unwrap());
        node.set_left(left.unwrap());
        Some(node)
    }

    
    fn parse_func(&mut self) -> Option<AstNode> { // Args As arrayand make right
        let mut name: String = String::new();
        let mut args: Vec<Box<AstNode>> = Vec::new();
        if let Some(i) = self.parse_ident() {
            name = i.value.to_string();
        } else {return None};
        if !self.eat_paren_start() {
            self.index -= 1;
            return None;
        }
        self.index -= 1;

        loop {
            if self.eat_paren_end() {
                break;
            }
            let arg = self.parse_arg();
            if let Some(i) = arg {
                args.push(Box::new(i));
            } else {
                break;
            }
            if !self.eat_comma() {
                self.index -= 1; // Dont know if this is right
                break;
                // maybe error
            }
            self.index -= 1;
        }
        let mut node = AstNode::new(name, AstTypes::FUNC);
        node.set_args(args);
        Some(node)
    }

    fn parse_arg(&mut self) -> Option<AstNode> {
        Some(self.parse_curr())
    }
}