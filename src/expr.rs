pub enum Expr {
    This(SourceLocation),
    Literal(Literal),
    Unary(UniOp, Box<Expr>),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Grouping(Box<Expr>),
}

/*
REGION: Expressions
*/

#[derive(Debug, Clone, Copy)]
pub struct SourceLocation {
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Copy, Clone)]
pub struct UnaryOp {
    pub u_type: UnaryOpTy,
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOpTy {
    Minus,
    Bang,
}

#[derive(Debug, Copy, Clone)]
struct BinOp {
    b_type: BinOpType,
    line: usize,
    column: i64,
}

#[derive(Debug, Copy, Clone)]
pub enum BinOpType {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Null,
}
