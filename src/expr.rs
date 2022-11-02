#[derive(Debug, Clone)]
pub enum Expr {
    This(SourceLocation),
    Literal(Literal),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone, Copy)]
pub struct SourceLocation {
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Copy, Clone)]
pub struct UnaryOp {
    pub u_type: UnaryOpType,
    pub line: usize,
    pub column: i64,
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOpType {
    Minus,
    Bang,
}

#[derive(Debug, Copy, Clone)]
pub struct BinOp {
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
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Null,
}
