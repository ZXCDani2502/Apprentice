use std::fmt;

use crate::expr::{self, Expr, Literal};
use crate::token::{self, Token, TokenType};

#[derive(Default)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub enum SyntaxError {
    UnexpectedToken(Token),
    TokenMismatch {
        expected: TokenType,
        found: Token,
        maybe_err: Option<String>,
    },
    InvalidTokenInBinaryOp {
        token_type: TokenType,
        line: usize,
        column: i64,
    },
    InvalidTokenInUnaryOp {
        token_type: TokenType,
        line: usize,
        column: i64,
    },
    ExpectedExpression {
        // an expression token was expected
        token_type: TokenType,
        line: usize,
        column: i64,
    },
}

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            SyntaxError::UnexpectedToken(token) => write!(
                f,
                "[line: {}, Column: {}] Unexpected token {:?} found for this place",
                token.line, token.column, token
            ),
            SyntaxError::TokenMismatch {
                expected,
                found,
                maybe_err,
            } => {
                write!(
                    f,
                    "[line: {}, Column: {}] Expected token {:?} but found {:?}",
                    found.line, found.column, expected, found.token_type,
                )?;
                if let Some(maybe_err) = maybe_err {
                    write!(f, ": {}", maybe_err)?;
                }
                fmt::Result::Ok(())
            }
            SyntaxError::InvalidTokenInBinaryOp {
                token_type,
                line,
                column,
            } => write!(
                f,
                "[line: {}, Column: {}] Invalid Binary Operator: {:?}",
                line, column, token_type
            ),
            SyntaxError::InvalidTokenInUnaryOp {
                token_type,
                line,
                column,
            } => write!(
                f,
                "[line: {}, Column: {}] Invalid Unary Operator: {:?}",
                line, column, token_type
            ),
            SyntaxError::ExpectedExpression {
                token_type,
                line,
                column,
            } => write!(
                f,
                "[line: {}, Column: {}] Expected Expression, found {:?}",
                line, column, token_type,
            ),
        }
    }
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

// function that allows external usage of the parser
pub fn parse(tokens: Vec<Token>) -> Result<Expr, SyntaxError> {
    let mut p = Parser {
        tokens,
        ..Default::default()
    };
    let parse_result = p.parse();

    match parse_result {
        Ok(result) => {
            // should be the end of the file, if it isn't the parser got stuck
            if !p.is_at_end() {
                let token = &p.tokens[p.current];
                Err(SyntaxError::UnexpectedToken(token.clone()))
            } else {
                Ok(result)
            }
        }
        Err(err) => Err(err),
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<Expr, SyntaxError> {
        // let ex = self.expression();
        // if ex.is_ok() {
        //     return ex;
        // } else {
        //     ex
        // }
        // that's the same as
        self.expression()
    }

    pub fn expression(&mut self) -> Result<Expr, SyntaxError> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr: Expr = self.comparison()?;

        while self.match_one_of(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator: Token = self.previous().clone();
            let right = Box::new(self.comparison()?);

            let binop_maybe = Parser::op_token_to_binop(&operator);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = Expr::Binary(left, binop, right);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr: Expr = self.term()?;

        while self.match_one_of(vec![
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let operator: Token = self.previous().clone();
            let right = Box::new(self.term()?);

            let binop_maybe = Parser::op_token_to_binop(&operator);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = Expr::Binary(left, binop, right);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(expr)
    }

    pub fn term(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr: Expr = self.factor()?;

        while self.match_one_of(vec![TokenType::Plus, TokenType::Minus]) {
            let operator: Token = self.previous().clone();
            let right = Box::new(self.factor()?);

            let binop_maybe = Parser::op_token_to_binop(&operator);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = Expr::Binary(left, binop, right);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(expr)
    }

    pub fn factor(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr: Expr = self.unary()?;

        while self.match_one_of(vec![TokenType::Star, TokenType::Slash]) {
            let operator: Token = self.previous().clone();
            let right = Box::new(self.unary()?);

            let binop_maybe = Parser::op_token_to_binop(&operator);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = Expr::Binary(left, binop, right);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(expr)
    }

    pub fn unary(&mut self) -> Result<Expr, SyntaxError> {
        while self.match_one_of(vec![TokenType::Minus, TokenType::Bang]) {
            let operator: Token = self.previous().clone();
            let right = Box::new(self.unary()?); // might change to not allow -- or !!

            let uniop_maybe = Parser::op_token_to_uniop(&operator);

            match uniop_maybe {
                Ok(uniop) => {
                    let expr = Expr::Unary(uniop, right);
                    return Ok(expr);
                }
                Err(e) => return Err(e),
            }
        }
        self.primary()
    }

    pub fn primary(&mut self) -> Result<Expr, SyntaxError> {
        if self.matches(TokenType::False) {
            return Ok(Expr::Literal(Literal::False));
        }
        if self.matches(TokenType::True) {
            return Ok(Expr::Literal(Literal::True));
        }
        if self.matches(TokenType::Null) {
            return Ok(Expr::Literal(Literal::Null));
        }
        if self.matches(TokenType::Number) {
            match &self.previous().literal {
                Some(token::Literal::Num(n)) => return Ok(Expr::Literal(Literal::Number(*n))),
                Some(l) => panic!(
                    "internal error in parser: when parsing number, found literal {:?}",
                    l
                ),
                None => panic!("internal error in parser: when parsing number, found no literal"),
            }
        }
        if self.matches(TokenType::String) {
            match &self.previous().literal {
                Some(token::Literal::Str(s)) => {
                    return Ok(Expr::Literal(Literal::String(s.clone())))
                }
                Some(l) => panic!(
                    "parser internal error: when parsing string, found literal {:?}",
                    l
                ),
                None => panic!("parser internal error: when parsing string, found no literal"),
            }
        }
        if self.matches(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        Err(SyntaxError::ExpectedExpression {
            token_type: self.peek().token_type,
            line: self.peek().line,
            column: self.peek().column,
        })
    }

    // might not be necessary

    // fn error(&self, token: Token, message: &str) -> Error {
    //     crate::t_error(token, message);
    //     Error
    // }

    // helper functions
    #[allow(unused)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class => (),
                TokenType::Func => (),
                TokenType::Var => (),
                TokenType::If => (),
                TokenType::For => (),
                TokenType::While => (),
                TokenType::Print => (),
                TokenType::Return => return,
                _ => todo!(),
            }

            self.advance();
        }
    }

    fn op_token_to_binop(op: &Token) -> Result<expr::BinaryOp, SyntaxError> {
        match op.token_type {
            TokenType::EqualEqual => Ok(expr::BinaryOp {
                b_type: expr::BinOpType::EqualEqual,
                line: op.line,
                column: op.column,
            }),
            TokenType::BangEqual => Ok(expr::BinaryOp {
                b_type: expr::BinOpType::NotEqual,
                line: op.line,
                column: op.column,
            }),
            TokenType::Less => Ok(expr::BinaryOp {
                b_type: expr::BinOpType::Less,
                line: op.line,
                column: op.column,
            }),
            TokenType::LessEqual => Ok(expr::BinaryOp {
                b_type: expr::BinOpType::LessEqual,
                line: op.line,
                column: op.column,
            }),
            TokenType::Greater => Ok(expr::BinaryOp {
                b_type: expr::BinOpType::Greater,
                line: op.line,
                column: op.column,
            }),
            TokenType::GreaterEqual => Ok(expr::BinaryOp {
                b_type: expr::BinOpType::GreaterEqual,
                line: op.line,
                column: op.column,
            }),
            TokenType::Plus => Ok(expr::BinaryOp {
                b_type: expr::BinOpType::Add,
                line: op.line,
                column: op.column,
            }),
            TokenType::Minus => Ok(expr::BinaryOp {
                b_type: expr::BinOpType::Sub,
                line: op.line,
                column: op.column,
            }),
            TokenType::Star => Ok(expr::BinaryOp {
                b_type: expr::BinOpType::Mult,
                line: op.line,
                column: op.column,
            }),
            TokenType::Slash => Ok(expr::BinaryOp {
                b_type: expr::BinOpType::Div,
                line: op.line,
                column: op.column,
            }),
            _ => Err(SyntaxError::InvalidTokenInBinaryOp {
                token_type: op.token_type,
                line: op.line,
                column: op.column,
            }),
        }
    }

    fn op_token_to_uniop(op: &Token) -> Result<expr::UnaryOp, SyntaxError> {
        match op.token_type {
            TokenType::Bang => Ok(expr::UnaryOp {
                u_type: expr::UniOpType::Bang,
                line: op.line,
                column: op.column,
            }),
            TokenType::Minus => Ok(expr::UnaryOp {
                u_type: expr::UniOpType::Minus,
                line: op.line,
                column: op.column,
            }),
            _ => Err(SyntaxError::InvalidTokenInUnaryOp {
                token_type: op.token_type,
                line: op.line,
                column: op.column,
            }),
        }
    }

    fn match_one_of(&mut self, types: Vec<TokenType>) -> bool {
        for t in types.iter() {
            if self.matches(*t) {
                return true;
            }
        }
        false
    }

    fn matches(&mut self, t: TokenType) -> bool {
        if self.check(t) {
            self.advance();
            return true;
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }

        self.previous()
    }

    fn consume(&mut self, t: TokenType, message: &str) -> Result<&Token, SyntaxError> {
        if self.check(t) {
            return Ok(self.advance());
        }
        Err(SyntaxError::TokenMismatch {
            expected: t,
            found: self.peek().clone(),
            maybe_err: Some(message.into()),
        })
    }

    fn check(&self, t: TokenType) -> bool {
        // true if the input equals the next token
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == t
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
