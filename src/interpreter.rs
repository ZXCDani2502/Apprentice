#![allow(unused)]
use crate::expr::{self, BinOpType, Expr, Literal, UniOpType};

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(lit) => Ok(self.interpret_literal(lit)),
            Expr::Grouping(e) => self.interpret_expr(e),
            Expr::Unary(op, e) => self.interpret_unary(*op, e),
            Expr::Binary(left, op, right) => self.interpret_binary(*op, left, right),

            _ => todo!(),
        }
    }

    fn interpret_literal(&self, lit: &Literal) -> Value {
        match lit {
            Literal::Number(n) => Value::Number(*n),
            Literal::String(s) => Value::String(s.clone()),
            Literal::True => Value::Bool(true),
            Literal::False => Value::Bool(false),
            Literal::Null => Value::Null,
        }
    }

    fn interpret_unary(&mut self, op: expr::UnaryOp, e: &Expr) -> Result<Value, String> {
        let val = self.interpret_expr(e)?;

        match (op.u_type, &val) {
            (UniOpType::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
            (UniOpType::Bang, Value::Bool(b)) => Ok(Value::Bool(!b)),
            (UniOpType::Minus, _) => Err("NaN".to_string()),
            (UniOpType::Bang, _) => Err("Not a boolean".to_string()),
            // to do more errorable options
        }
    }

    fn interpret_binary(
        &mut self,
        op: expr::BinaryOp,
        left: &Expr,
        right: &Expr,
    ) -> Result<Value, String> {
        let l = self.interpret_expr(left)?;
        let r = self.interpret_expr(right)?;

        match (&l, op.b_type, &r) {
            (Value::Number(l), BinOpType::Less, Value::Number(r)) => Ok(Value::Bool(l < r)),
            (Value::Number(l), BinOpType::LessEqual, Value::Number(r)) => Ok(Value::Bool(l <= r)),
            (Value::Number(l), BinOpType::Greater, Value::Number(r)) => Ok(Value::Bool(l > r)),
            (Value::Number(l), BinOpType::GreaterEqual, Value::Number(r)) => {
                Ok(Value::Bool(l >= r))
            }
            (Value::Number(l), BinOpType::Sub, Value::Number(r)) => Ok(Value::Number(l - r)),
            (Value::Number(l), BinOpType::Add, Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::Number(l), BinOpType::Mult, Value::Number(r)) => Ok(Value::Number(l * r)),
            (Value::Number(l), BinOpType::Div, Value::Number(r)) => {
                if *r == 0.0 {
                    Err(format!(
                        "[line: {} Column: {}] Can't divide by zero",
                        op.line, op.column,
                    ))
                } else {
                    Ok(Value::Number(l / r))
                }
            }
            (Value::String(l), BinOpType::Add, Value::String(r)) => {
                Ok(Value::String(format!("{l}{r}")))
            }
            (_, BinOpType::EqualEqual, _) => Ok(Value::Bool(Interpreter::equals(&l, &r))),
            (_, BinOpType::NotEqual, _) => Ok(Value::Bool(Interpreter::equals(&l, &r))),

            _ => Err(todo!()),
        }
    }

    // helper functions

    fn equals(left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(n1), Value::Number(n2)) => (n1 - n2).abs() < f64::EPSILON,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Null, Value::Null) => true,
            (_, _) => false,
        }
    }
}
