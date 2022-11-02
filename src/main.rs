use crate::token::Token;
use std::fs;

mod expr;
mod parser;
mod scanner;
mod token;

fn main() {
    run(String::from("test"));
}

fn run(source: String) {
    //temporary way of creating the scanner
    let code = fs::read_to_string(format!("src/{}.aprn", source)).expect("can't read file");
    let result = scanner::scan(code);

    let mut tokens: Vec<Token> = Vec::new();
    if let Err(e) = result {
        error(e.line, e.column, e.message.as_str());
    } else {
        tokens = result.unwrap();
    }

    // for now just print the tokens
    for token in tokens {
        println!("{token:?}");
    }
}

fn error(line: usize, column: i64, message: &str) {
    report(line, column, "", message);
}

fn report(line: usize, column: i64, place: &str, message: &str) {
    panic!("[line {line}, column {column}] Error {place}: {message}");
    //had_error = true;
}
