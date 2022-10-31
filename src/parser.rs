use crate::expr;
use crate::scanner;

use std::fmt;

#[derive(Default)]
struct Parser {
    tokens: Vec<scanner::Token>,
    current: usize,
}
