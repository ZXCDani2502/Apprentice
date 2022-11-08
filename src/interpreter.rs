use crate::expr::{self, Expr, UniOpType};
use crate::value::Value;
pub struct Interpreter {}

impl Interpreter {
    pub fn interpret_expr(&self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(lit) => Value::Literal(lit),
            Expr::Grouping(e) => self.interpret_expr(e),
            Expr::Unary(op, e) => self.interpret_unary(op, e),

            _ => todo!(),
        }
    }

    fn interpret_unary(&self, op: expr::UnaryOp, e: Box<Expr>) -> Result<Value, String> {
        let val = self.interpret_expr(e);

        match (op.u_type, val) {
            (UniOpType::Minus, Value::Number(val)) => Ok(Value::Number(-val)),
            (UniOpType::Bang, Value::Bool(b)) => Ok(Value::Bool(!b)),
            (UniOpType::Minus, _) => Err("NaN".to_string()),
            (UniOpType::Bang, _) => Err("Not a boolean".to_string()),
        }
    }
}
