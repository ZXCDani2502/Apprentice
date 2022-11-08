use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    //This(SourceLocation),
    Literal(Literal),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(Debug, Copy, Clone)]
pub struct SourceLocation {
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Copy, Clone)]
pub struct UnaryOp {
    pub u_type: UniOpType,
    pub line: usize,
    pub column: i64,
}

#[derive(Debug, Copy, Clone)]
pub enum UniOpType {
    Minus,
    Bang,
}
impl fmt::Display for UniOpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            UniOpType::Minus => write!(f, "-"),
            UniOpType::Bang => write!(f, "!"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BinaryOp {
    pub b_type: BinOpType,
    pub line: usize,
    pub column: i64,
}

#[derive(Debug, Copy, Clone)]
pub enum BinOpType {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Add,
    Sub,
    Mult,
    Div,
}
impl fmt::Display for BinOpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            BinOpType::EqualEqual => write!(f, "=="),
            BinOpType::NotEqual => write!(f, "!="),
            BinOpType::Less => write!(f, "<"),
            BinOpType::LessEqual => write!(f, "<="),
            BinOpType::Greater => write!(f, ">"),
            BinOpType::GreaterEqual => write!(f, ">="),
            BinOpType::Add => write!(f, "+"),
            BinOpType::Sub => write!(f, "-"),
            BinOpType::Mult => write!(f, "*"),
            BinOpType::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Null,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Literal::Number(n) => write!(f, "{n}"),
            Literal::String(s) => write!(f, "{s}"),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Null => write!(f, "null"),
        }
    }
}
