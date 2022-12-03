use crate::parser::exprstmt::Expr;

pub fn pr(expr: Expr) {
    println!("{}", format(expr));
}

fn format(expr: Expr) -> String {
    match expr {
        Expr::Grouping(expr) => parenthesize("group".to_string(), expr),
        Expr::Unary(op, expr) => parenthesize(format!("{}", op.u_type), expr),
        Expr::Binary(left, op, right) => parenthesize_bin(format!("{}", op.b_type), left, right),
        Expr::Ternary(bool, if_, else_) => parenthesize_tri(bool, if_, else_),
        Expr::Literal(value) => return format!("{value}"),
        Expr::Variable(name) => todo!(),
        Expr::Assignment(sym, expr) => todo!(),
    }
}

fn parenthesize_tri(bool: Box<Expr>, if_: Box<Expr>, else_: Box<Expr>) -> String {
    let s: String = String::from(format!(
        "({} ? {} : {})",
        format(*bool),
        format(*if_),
        format(*else_)
    ));
    s
}

fn parenthesize_bin(name: String, left: Box<Expr>, right: Box<Expr>) -> String {
    let s: String = String::from(format!("({} {} {})", format(*left), name, format(*right)));
    s
}

fn parenthesize(name: String, expr: Box<Expr>) -> String {
    let s: String = String::from(format!("({} {})", name, format(*expr)));
    s
}
