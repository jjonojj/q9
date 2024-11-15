use std::fmt::Display;

use crate::parser::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    Lit(ExprValue),
    Fn(String, Vec<Expr>),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum ExprValue {
    Num(f64),
}

impl Display for ExprValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(num) => write!(f, "number(value: {num})"),
            /* _ => write!(f, "unexpected"), */ // add this when more data types get added
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Div,
    Mul,
}

pub struct ExprHandler {
    left: Option<Expr>,
    operator: Option<BinOp>,
}

impl Parser {
    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.cur.clone() {
            Token::Num(num) => { self.advance(); Ok(Expr::Lit(ExprValue::Num(parse_f64(&num)?))) },
            Token::Str(ident) => {
                match self.peek {
                    Token::LParen => {
                        self.advance();
                        self.check(Token::LParen)?;
                        self.advance();
                        // parse args
                        let mut args = Vec::new();
                        while self.cur != Token::RParen {
                            args.push(self.parse_expr()?);
                            if self.cur == Token::RParen { break };
                            self.check(Token::Comma)?;
                            self.advance();
                        }
                        self.check(Token::RParen)?;
                        self.advance();
                        Ok(Expr::Fn(ident.to_owned(), args)) 
                    }
                    _ => {
                        self.advance();
                        Ok(Expr::Ident(ident.to_owned()))
                    }
                }
            },
            Token::Plus => { todo!() },
            Token::Minus => { todo!() },
            Token::Slash => { todo!() },
            Token::Star => { todo!() },
            Token::SemiC => { todo!() },
            _ => return Err(format!("invalid expression: {}", self.cur))
        }
    }
}

fn parse_f64(num: &str) -> Result<f64, String> {
    num.parse::<f64>()
        .ok()
        .ok_or(format!("could not parse {num} to a number."))
}

fn binop(lval: ExprValue, op: BinOp, rval: ExprValue) -> Result<ExprValue, String> {
    match (lval, rval) {
        (ExprValue::Num(lhs), ExprValue::Num(rhs)) => match op {
            BinOp::Add => Ok(ExprValue::Num(lhs + rhs)),
            BinOp::Sub => Ok(ExprValue::Num(lhs - rhs)),
            BinOp::Mul => Ok(ExprValue::Num(lhs * rhs)),
            BinOp::Div => {
                if rhs != 0f64 {
                    Ok(ExprValue::Num(lhs / rhs))
                } else {
                    Err("cannot divide by zero".to_string())
                }
            }
        },
        /* _ => Err(format!("cannot do the mafhs with type {} and {}.", lval, rval))*/ // add this later when more value types are added
    }
}
