use crate::expr;
use crate::token;

use std::fmt;

#[derive(Default)]
struct Parser {
    tokens: Vec<token::Token>,
    current: usize,
}

impl Parser {
    pub fn expression(&self) -> expr::Expr {
        equality()
    }

    pub fn equality(&self) -> expr::Expr {
        let expr: expr::Expr = comparison();
    }

    pub fn comparison(&self) -> expr::Expr {}
}
