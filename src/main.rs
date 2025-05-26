mod token_type;
mod token;
mod scanner;
mod parser;
mod ast;
mod interpreter;

use std::env;
use std::io::{self, BufRead, Write};
use std::fs;
use std::process;

use crate::scanner::Scanner;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::parser::Parser;
use crate::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n_args = args.len();

    if n_args > 2 {
        panic!("Usage: jlox [script]");
    } else if n_args == 2 {
        run_file(&args[1]);
    } else if n_args == 1{
        run_prompt();
    } else {
        process::exit(0);
    }
}

fn run_file(path: &String) {
    let content: String = fs::read_to_string(path).expect("Unable to read file");
    run(content);
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        handle.read_line(&mut buffer).expect("Error reading input");
        if buffer == "\n" {
            break;
        }
        run(buffer);
    }
}

fn run(source: String) {
    let mut scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens(); // TODO: Have this return iterator

    for token in tokens.iter() {
        println!("{}", token.to_string());
    } 

    let mut parser: Parser = Parser::new(tokens);
    let expression_opt = parser.parse();


    if expression_opt.is_none() {
        return 
    }
    
    let expression = expression_opt.unwrap();
    
    let interpreter = Interpreter{};
    interpreter.interpret(expression);
}

pub fn error(line: usize, message: &'static str) {
    report(line, "".to_string(), message);
}

pub fn token_error(token: Token, message: &'static str) {
    if token.token_type == TokenType::EOF{
        report(token.line, " at end".to_string(), message);
    } else {
        report(token.line, format!(" at '{}'", token.lexeme), message);
    }
}

fn report(line: usize, loc: String, message: &'static str) {
    eprintln!("[line: {line}] Error {loc}: {message}");
}