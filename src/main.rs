use crate::scanner::{Scanner, Token};
use std::fs;

mod scanner;

fn main() {
    run_file(String::from("test"));
}

fn run_file(path: String) {
    // -- some logic --
    run(path);
}

fn run(source: String) {
    //temporary way of creating the scanner
    let mut scanner: Scanner = Scanner::default();
    let tokens: Vec<Token> = scanner.scan_tokens(source);

    // for now just print the tokens
    for token in tokens {
        println!("{token:?}");
    }
}

fn error(line: usize, column: i64, message: &str) {
    report(line, column, "", message);
}

fn report(line: usize, column: i64, place: &str, message: &str) {
    panic!("[line {line}] Error {place}: {message}");
    //had_error = true;
}
