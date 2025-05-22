use std::env;
use std::io::{self, BufRead, Write};
use std::fs;
use std::process;

mod token_type;
mod token;
mod scanner;

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
    run(&content);
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        handle.read_line(&mut buffer).expect("Error reading input");
        if buffer == "\n" {
            break;
        }
        run(&buffer);
        buffer.clear();
    }
}

fn run(source: &String) {
    let mut scanner: scanner::Scanner = scanner::Scanner::new(&source);
    let tokens: &Vec<token::Token> = scanner.scan_tokens(); // TODO: Have this return iterator

    for token in tokens.into_iter() {
        println!("{}", token.to_string());
    } 
}

fn error(line: usize, message: &'static str) {
    report(line, "", message);
}

fn report(line: usize, loc: &'static str, message: &'static str) {
    eprintln!("[line: {line}] Error {loc}: {message}");
}
