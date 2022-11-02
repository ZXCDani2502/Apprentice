use crate::expr;
use crate::token;

#[derive(Default)]
struct Parser {
    tokens: Vec<token::Token>,
    current: usize,
}

pub enum Error {
    InvalidTokenInBinaryOp {
        token_type: token::TokenType,
        line: usize,
        column: i64,
    },
    InvalidTokenInUnaryOp {
        token_type: token::TokenType,
        line: usize,
        column: i64,
    },
    TokenMismatch {
        expected: token::TokenType,
        found: token::Token,
        maybe_err: Option<String>,
    },
}

/*

Recursive descent using this grammar

expression     = equality ;
equality       = comparison ( ( "!=" | "==" ) comparison )* ;
comparison     = term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           = factor ( ( "-" | "+" ) factor )* ;
factor         = unary ( ( "/" | "*" ) unary )* ;
unary          = ( "!" | "-" ) unary
               | primary ;
primary        = NUMBER | STRING | "true" | "false" | "null"
               | "(" expression ")" ;

*/

impl Parser {
    pub fn expression(&mut self) -> expr::Expr {
        equality()
    }

    pub fn equality(&mut self) -> Result<expr::Expr, Error> {
        let mut expr: expr::Expr = self.comparison()?;

        while self.match_one_of(vec![
            token::TokenType::EqualEqual,
            token::TokenType::BangEqual,
        ]) {
            let operator: token::Token = self.previous().clone();
            let right = Box::new(self.comparison()?);

            let binop_maybe = Parser::op_token_to_binOp(&operator);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<expr::Expr, Error> {
        let mut expr: expr::Expr = self.term()?;

        while self.match_one_of(vec![
            token::TokenType::Less,
            token::TokenType::LessEqual,
            token::TokenType::Greater,
            token::TokenType::GreaterEqual,
        ]) {
            let operator: token::Token = self.previous().clone();
            let right = Box::new(self.term()?);

            let binop_maybe = Parser::op_token_to_binOp(&operator);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(expr)
    }

    pub fn term(&mut self) -> Result<expr::Expr, Error> {
        let mut expr: expr::Expr = self.factor()?;

        while self.match_one_of(vec![token::TokenType::Plus, token::TokenType::Minus]) {
            let operator: token::Token = self.previous().clone();
            let right = Box::new(self.factor()?);

            let binop_maybe = Parser::op_token_to_binOp(&operator);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(expr)
    }

    pub fn factor(&mut self) -> Result<expr::Expr, Error> {
        let mut expr: expr::Expr = self.unary()?;

        while self.match_one_of(vec![token::TokenType::Star, token::TokenType::Slash]) {
            let operator: token::Token = self.previous().clone();
            let right = Box::new(self.unary()?);

            let binop_maybe = Parser::op_token_to_binOp(&operator);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = expr::Expr::Binary(left, binop, right);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(expr)
    }

    pub fn unary(&mut self) -> Result<expr::Expr, Error> {
        while self.match_one_of(vec![token::TokenType::Minus, token::TokenType::Bang]) {
            let operator: token::Token = self.previous().clone();
            let right = Box::new(self.unary()?); // might change to not allow -- or !!

            let uniop_maybe = Parser::op_token_to_uniOp(&operator);

            match uniop_maybe {
                Ok(uniop) => {
                    let expr = expr::Expr::Unary(uniop, right);
                    return Ok(expr);
                }
                Err(e) => return Err(e),
            }
        }
        primary()
    }

    pub fn primary(&mut self) -> Result<expr::Expr, Error> {
        if self.matches(token::TokenType::False) {
            return Ok(expr::Expr::Literal(expr::Literal::False));
        }
        if self.matches(token::TokenType::True) {
            return Ok(expr::Expr::Literal(expr::Literal::True));
        }
        if self.matches(token::TokenType::Null) {
            return Ok(expr::Expr::Literal(expr::Literal::Null));
        }
        if self.matches(token::TokenType::Number) {
            match &self.previous().literal {
                Some(token::Literal::Num(n)) => {
                    return Ok(expr::Expr::Literal(expr::Literal::Number(*n)))
                }
                Some(l) => panic!(
                    "internal error in parser: when parsing number, found literal {:?}",
                    l
                ),
                None => panic!("internal error in parser: when parsing number, found no literal"),
            }
        }
        if self.matches(token::TokenType::String) {
            match &self.previous().literal {
                Some(token::Literal::Str(s)) => {
                    return Ok(expr::Expr::Literal(expr::Literal::String(*s)))
                }
                Some(l) => panic!(
                    "parser internal error: when parsing string, found literal {:?}",
                    l
                ),
                None => panic!("parser internal error: when parsing string, found no literal"),
            }
        }
        if self.matches(token::TokenType::LeftParen) {
            let mut expr = self.expression();
            self.consume(
                token::TokenType::RightParen,
                "Expected ')' after expression",
            );
            return Ok(expr::Expr::Grouping(Box::new(expr)));
        }
    }

    // helper functions

    fn op_token_to_binOp(op: &token::Token) -> Result<expr::BinOp, Error> {
        match op.token_type {
            token::TokenType::EqualEqual => Ok(expr::BinOp {
                b_type: expr::BinOpType::EqualEqual,
                line: op.line,
                column: op.column,
            }),
            token::TokenType::BangEqual => Ok(expr::BinOp {
                b_type: expr::BinOpType::NotEqual,
                line: op.line,
                column: op.column,
            }),
            token::TokenType::Less => Ok(expr::BinOp {
                b_type: expr::BinOpType::Less,
                line: op.line,
                column: op.column,
            }),
            token::TokenType::LessEqual => Ok(expr::BinOp {
                b_type: expr::BinOpType::LessEqual,
                line: op.line,
                column: op.column,
            }),
            token::TokenType::Greater => Ok(expr::BinOp {
                b_type: expr::BinOpType::Greater,
                line: op.line,
                column: op.column,
            }),
            token::TokenType::GreaterEqual => Ok(expr::BinOp {
                b_type: expr::BinOpType::GreaterEqual,
                line: op.line,
                column: op.column,
            }),
            token::TokenType::Plus => Ok(expr::BinOp {
                b_type: expr::BinOpType::Add,
                line: op.line,
                column: op.column,
            }),
            token::TokenType::Minus => Ok(expr::BinOp {
                b_type: expr::BinOpType::Sub,
                line: op.line,
                column: op.column,
            }),
            token::TokenType::Star => Ok(expr::BinOp {
                b_type: expr::BinOpType::Mult,
                line: op.line,
                column: op.column,
            }),
            token::TokenType::Slash => Ok(expr::BinOp {
                b_type: expr::BinOpType::Div,
                line: op.line,
                column: op.column,
            }),
            _ => Err(Error::InvalidTokenInBinaryOp {
                token_type: op.token_type,
                line: op.line,
                column: op.column,
            }),
        }
    }

    fn op_token_to_uniOp(op: &token::Token) -> Result<expr::UnaryOp, Error> {
        match op.token_type {
            token::TokenType::Bang => Ok(expr::UnaryOp {
                u_type: expr::UnaryOpType::Bang,
                line: op.line,
                column: op.column,
            }),
            token::TokenType::Minus => Ok(expr::UnaryOp {
                u_type: expr::UnaryOpType::Minus,
                line: op.line,
                column: op.column,
            }),
            _ => Err(Error::InvalidTokenInUnaryOp {
                token_type: op.token_type,
                line: op.line,
                column: op.column,
            }),
        }
    }

    fn match_one_of(&mut self, types: Vec<token::TokenType>) -> bool {
        for t in types.iter() {
            if self.matches(*t) {
                return true;
            }
        }
        false
    }

    fn matches(&mut self, t: token::TokenType) -> bool {
        if self.check(t) {
            self.advance();
            return true;
        }
        false
    }

    fn advance(&mut self) -> &token::Token {
        if !self.is_at_end() {
            self.current += 1
        }

        self.previous()
    }

    fn consume(&mut self, t: token::TokenType, message: &str) -> Result<&token::Token, Error> {
        if self.check(t) {
            return Ok(self.advance());
        }
        Err(Error::TokenMismatch {
            expected: t,
            found: self.peek().clone(),
            maybe_err: Some(message.into()),
        })
    }

    fn check(&self, t: token::TokenType) -> bool {
        // true if the input equals the next token
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == t
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == token::TokenType::Eof
    }

    fn peek(&self) -> &token::Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &token::Token {
        &self.tokens[self.current - 1]
    }
}
