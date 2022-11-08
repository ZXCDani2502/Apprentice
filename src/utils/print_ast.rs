use crate::expr::{self, Expr};

pub fn pr(expr: expr::Expr) {
    println!("{}", format(expr));
}

fn format(expr: expr::Expr) -> String {
    match expr {
        Expr::Grouping(expr) => parenthesize("group".to_string(), expr),
        Expr::Unary(op, expr) => parenthesize(format!("{}", op.u_type), expr),
        Expr::Binary(left, op, right) => parenthesize_bin(format!("{}", op.b_type), left, right),
        expr::Expr::Literal(value) => return format!("{value}"),
    }
}

fn parenthesize_bin(name: String, left: Box<expr::Expr>, right: Box<expr::Expr>) -> String {
    let s: String = String::from(format!("({} {} {})", format(*left), name, format(*right)));
    s
}

fn parenthesize(name: String, expr: Box<expr::Expr>) -> String {
    let s: String = String::from(format!("({} {})", name, format(*expr)));
    s
}
