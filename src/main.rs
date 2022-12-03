use crate::scanner::token::Token;
// use crate::utils::*;
use std::fs;

mod interpreter;
mod parser;
mod scanner;
pub mod utils;

fn main() {
    run(String::from("test"));
}

fn run(source: String) {
    let mut had_error = false;
    //temporary way of creating the scanner
    let code = fs::read_to_string(format!("src/tests/{}.aprn", source)).expect("can't read file");
    let scan_result = scanner::scan(code);

    let mut tokens: Vec<Token> = Vec::new();
    if let Err(e) = scan_result {
        had_error = true;
        error(e.line, e.column, e.message.as_str());
    } else {
        tokens = scan_result.unwrap();
    }

    // print_token::pr(&tokens);

    let parse_result = parser::parse(tokens);

    let ast = match parse_result {
        Ok(stmts) => {
            stmts
            //print_ast::pr(todo!());
        }
        Err(err) => {
            had_error = true;
            println!("{err:?}");
            vec![]
        }
    };

    if had_error {
        return;
    }

    let runtime_result = interpreter::interpret(&ast);
    match runtime_result {
        Ok(_) => return,
        Err(s) => println!("{s:?}"),
    }
}

fn error(line: usize, column: i64, message: &str) {
    report(line, column, "", message);
}

// fn t_error(token: Token, message: &str) {
//     if token.token_type == token::TokenType::Eof {
//         report(token.line, token.column, " at end", message);
//     } else {
//         report(
//             token.line,
//             token.column,
//             format!(" at '{}'", String::from_utf8(token.lexeme).unwrap()).as_str(),
//             message,
//         );
//     }
// }

fn report(line: usize, column: i64, place: &str, message: &str) {
    panic!("[line: {line}, column: {column}] Error {place}: {message}");
    //had_error = true;
}
