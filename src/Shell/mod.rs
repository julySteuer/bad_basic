use std::collections::HashMap;
use std::io::{stdin, Write};
use std::io;
use std::fs;
use crate::Parser::Ast::AstNode;
use crate::Tokenizer;
use crate::Parser;
use crate::Interpreter;

pub struct Shell {
    tokenizer: Tokenizer::Tokenizer
}

impl Shell {
    pub fn new() -> Shell {
        Shell {tokenizer: Tokenizer::Tokenizer::new_empty() }
    }

    pub fn env_starter() {
        let banner = fs::read_to_string("assets/banner.txt").unwrap();
        println!("{}", banner);
        println!("Bad Basic [version 0.0.1]");
        println!("Interpreted by this shitty script");
    }

    pub fn line_runnner() {
        // Make global Tokenizer let parser run on RUN command;
        print!("]");
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Error while reading From stdin");
        input = input.replace("\n", "");
        let mut tokenizer = Tokenizer::Tokenizer::new_from_stream(input);
        tokenizer.tokenize();
        let mut parser: Parser::Parser = Parser::Parser::new(tokenizer);
        parser.parse();
        let mut interpreter: Interpreter::Interpreter = Interpreter::Interpreter::new(parser);
        let res = interpreter.run_line();
        println!("{}", res);
    }

    pub fn cont_runner(&mut self){
        loop {
            self.tokenizer.clear();
            let mut line_map: HashMap<String, AstNode> = HashMap::new();
            loop {
                print!("]");
                io::stdout().flush().unwrap();
                let mut input: String = String::new();
                stdin().read_line(&mut input).expect("Error while reading From stdin");
                input = input.replace("\n", "");
                if input == "RUN" {
                    break;
                }
                let mut tokenizer = Tokenizer::Tokenizer::new_from_stream(input);
                tokenizer.tokenize();
                let mut parser = Parser::Parser::new(tokenizer);
                parser.parse();
                line_map.extend(parser.get_lines().clone());
            }
            let mut interpreter = Interpreter::Interpreter::new_from_map(&line_map);
            let res = interpreter.run();
            println!("> {}", res);
        }
    }
}