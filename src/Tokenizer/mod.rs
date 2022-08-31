use core::panic;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenTypes {
    NUMBER,
    STRING,
    ASSIGN,
    LINEBREAK,
    OP,
    WHITESPACE,
    LPAREN,
    RPAREN,
    COMMA
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenTypes
}
 // Indecies 0-4
lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"\d").unwrap();
    static ref STRING: Regex = Regex::new(r"\w").unwrap();
    static ref BINOP: Vec<char> = vec!['+', '-', '*', '/'];
}

#[derive(Debug, Clone)]
pub struct Tokenizer {
    index: usize,
    file_content: String,
    tokens: Vec<Token>
}

// Maybe tokenizer concat function
impl Tokenizer {
    pub fn new_from_file(filename: String) -> Tokenizer {
        Tokenizer { index: 0, file_content: fs::read_to_string(&filename).unwrap(), tokens: Vec::new() }
    }

    pub fn new_empty() -> Tokenizer {
        Tokenizer { index:0, file_content: String::new(), tokens: Vec::new()}
    }

    pub fn new_from_stream(inp: String) -> Tokenizer {
        Tokenizer { index: 0, file_content: inp, tokens: Vec::new() }
    }

    fn get_current(&self) -> char {
        self.file_content.chars().nth(self.index).unwrap()
    }

    fn is_number(&self) -> bool { // i think here 
        NUMBER.is_match(&self.get_current().to_string())
    }

    fn is_string(&self) -> bool {
        STRING.is_match(&self.get_current().to_string())
    }

    fn is_assign(&self) -> bool {
        self.get_current() == '='
    }

    fn is_bin_op(&self) -> bool {
        BINOP.contains(&self.get_current())
    }

    fn is_whitespace(&self) -> bool {
        self.get_current() == ' '
    }

    fn is_linebreak(&self) -> bool {
        self.get_current() == '\n'
    }

    fn is_lparen(&self) -> bool {
        self.get_current() == '('
    }

    fn is_rparen(&self) -> bool {
        self.get_current() == ')'
    }

    fn is_comma(&self) -> bool {
        self.get_current() == ','
    }

    fn check_type(&self) -> TokenTypes {
        if self.is_number() {TokenTypes::NUMBER}
        else if self.is_assign() {TokenTypes::ASSIGN}
        else if self.is_bin_op() {TokenTypes::OP}
        else if self.is_linebreak() {TokenTypes::LINEBREAK}
        else if self.is_string() {TokenTypes::STRING}
        else if self.is_whitespace() {TokenTypes::WHITESPACE}
        else if self.is_lparen() {TokenTypes::LPAREN}
        else if self.is_rparen() {TokenTypes::RPAREN}
        else if self.is_comma() {TokenTypes::COMMA}
        else {panic!("Unknown Token could not get token type at {}", self.index)}   
    }
    
    pub fn tokenize(&mut self) {
        loop {
            let new_token: Token = self.concat();
            self.tokens.push(new_token);
            if self.is_eos() {
                break;
            }
        }
    }

    pub fn concat_tokens(&mut self, tokens: &mut Vec<Token>) {
        self.tokens.append(tokens);
    }

    fn concat(&mut self) -> Token{
        let current_type: TokenTypes = self.check_type();
        let mut res : String = String::new();
        while self.check_type() == current_type {
            res.push(self.get_current());
            self.index += 1;
            if self.is_eos() { // Error here
                break;
            }
        }
        Token { value: res, token_type: current_type }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn is_eos(&self) -> bool {
        self.index >= self.file_content.len()
    }

    pub fn clear(&mut self) {
        self.tokens = Vec::new();
    }
}