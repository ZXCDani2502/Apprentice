use core::panic;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Int,
    Float,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Func,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Int(u64),
    Float(f64),
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
    pub line: usize,
    pub column: i64,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token {{ type: {:?}, lexeme: \"{}\", literal: {:?}, line: {:?}, col: {:?}}}",
            self.token_type,
            String::from_utf8(self.lexeme.clone()).unwrap(),
            self.literal,
            self.line,
            self.column
        )
    }
}

#[derive(Debug)]
pub struct Error {
    //error handling is done in main if the error value in the scanner is Some and not None
    pub what: String,
    pub line: usize,
    pub column: i64,
}

//the function that main calls which creates the scanner
#[allow(unused)]
pub fn scan(input: String) -> Result<Vec<Token>, Error> {
    let mut scanner: Scanner = Default::default();
    scanner.scan_tokens(input);

    match scanner.err {
        Some(err) => Err(err),
        None => Ok(scanner.tokens),
    }
}

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    err: Option<Error>,
    start: usize,
    current: usize,
    line: usize,
    column: i64,
    keywords: HashMap<String, TokenType>,
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            err: None,
            start: 0,
            current: 0,
            line: 1,
            column: -1,
            //create a hashmap from a list of tuples(saves a lot of boilerplate String::from)
            keywords: vec![
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("func", TokenType::Func),
                ("if", TokenType::If),
                ("null", TokenType::Null),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect(),
        }
    }
}

impl Scanner {
    //create a vec of all the tokens from an input
    pub fn scan_tokens(&mut self, input: String) -> Vec<Token> {
        self.source = input.into_bytes();

        let tokens: Vec<Token> = Vec::new();
        while !self.is_at_end() {
            // beginning of the lexeme
            self.start = self.current;
            self.scan_token();
        }

        //push the end of file token at the end of the input
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: Vec::new(),
            literal: None,
            line: self.line,
            column: self.column,
        });
        tokens
    }

    //identify the individual tokens and calls for their creation
    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            //single character tokens
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            //single or double character tokens
            '!' => {
                let matches = self.matches('=');
                self.add_token(if matches {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let matches = self.matches('=');
                self.add_token(if matches {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let matches = self.matches('=');
                self.add_token(if matches {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let matches = self.matches('=');
                self.add_token(if matches {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            //handle division and comments
            '/' => {
                let matches = self.matches('/');
                if matches {
                    while self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            //ingore white space
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.column = 0;
            }
            //strings
            '"' => self.string(),
            //invalid characters
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else {
                    self.err = Some(Error {
                        what: format!("Invalid character found: {c}"),
                        line: self.line,
                        column: self.column,
                    })
                }
            }
        }
    }

    //checks if the input character matches the next
    //character in the list and consumes it if true
    fn matches(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return true;
        }
        if char::from(self.source[self.current]) != c {
            return false;
        }
        self.current += 1;
        self.column += 1;
        true
    }

    //gets the next character in the list
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            char::from(self.source[self.current])
        }
    }

    //gets the character after the next character in the list
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            char::from(self.source[self.current + 1])
        }
    }

    //handles strings
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\\' {
                todo!(); //escape characters
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.err = Some(Error {
                what: "String needs to be closed".to_string(),
                line: self.line,
                column: self.column,
            })
        }

        //eliminated all other options
        assert!(self.peek() == '"');

        self.advance();

        self.add_token_literal(
            TokenType::String,
            Some(Literal::Str(
                //creates a string from bytes without the quotes
                String::from_utf8(self.source[self.start + 1..self.current - 1].to_vec()).unwrap(),
            )),
        );
    }

    //handles numbers
    fn number(&mut self) {
        let mut int = true;

        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            int = false;
            self.advance(); //consume the "."

            while self.peek().is_ascii_digit() {
                self.advance();
            }

            let val: f64 = String::from_utf8(self.source[self.start..self.current].to_vec())
                .unwrap()
                .parse()
                .unwrap();
            self.add_token_literal(TokenType::Float, Some(Literal::Float(val)))
        }

        if int {
            let val: u64 = String::from_utf8(self.source[self.start..self.current].to_vec())
                .unwrap()
                .parse()
                .unwrap();
            self.add_token_literal(TokenType::Int, Some(Literal::Int(val)))
        }
    }

    //move one character forward in the input
    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;

        char::from(self.source[self.current - 1])
    }

    //buffer function for non-literal tokens
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    //create and adds a token to the vec
    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_vec();

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
            column: self.column,
        })
    }

    // true if at the end of the input
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
