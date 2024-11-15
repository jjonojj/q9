#![allow(dead_code, unused)]
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use expr::{Expr, ExprValue};

use crate::parser::*;

#[derive(Clone)]
pub enum Value {
    Num(f64),
    Void,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(val) => write!(f, "number value (value: {})", val),
            Value::Void => write!(f, "void value"),
            _ => unreachable!(),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(val) => write!(f, "number value (value: {})", val),
            Value::Void => write!(f, "void value"),
            _ => unreachable!(),
        }
    }
}

impl From<ExprValue> for Value {
    fn from(value: ExprValue) -> Self {
        match value {
            ExprValue::Num(num) => Self::Num(num),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EvalVariable {
    name: String,
    val: Value,
}

#[derive(Clone)]
pub struct EvalFunction {
    name: String,
    body: Vec<Statement>,
    args: Vec<String>,
}

pub struct Q9Eval {
    pub prog: Program,
    vars: Vec<EvalVariable>,
    locals: Vec<EvalVariable>,
    fns: Vec<EvalFunction>,
    global: bool,
}

impl Q9Eval {
    pub fn new(prog: Program) -> Self {
        Self {
            prog,
            vars: Vec::new(),
            locals: Vec::new(),
            fns: Vec::new(),
            global: true,
        }
    }

    pub fn eval(&mut self, stmts: Vec<Statement>, fn_args: Option<Vec<EvalVariable>>) -> Value {
        if let Some(args_vec) = fn_args {
            self.locals.extend(args_vec);
        }
        for stmt in stmts {
            match stmt {
                Statement::FuncDef(nfunc, arg_names) => {
                    assert!(
                        self.global == true,
                        "cannot define function '{}' in non-global scope",
                        nfunc.name
                    );
                    if (self
                        .fns
                        .iter()
                        .filter(|func| func.name == nfunc.name)
                        .next()
                        .is_none())
                    {
                        self.fns.push(EvalFunction {
                            name: nfunc.name.to_owned(),
                            body: nfunc.body.body.clone(),
                            args: arg_names,
                        })
                    } else {
                        panic!("function '{}' already defined", nfunc.name);
                    }
                }
                Statement::VarDef(nname, nexpr) => {
                    if (self.get_val_from_var(nname.as_str()).is_none()) {
                        let var = EvalVariable {
                            name: nname.to_owned(),
                            val: self.eval_expr(nexpr.clone()),
                        };
                        if !self.global {
                            self.locals.push(var);
                        } else {
                            self.vars.push(var);
                        }
                    } else {
                        panic!(
                            "variable '{}' is already defined; redefinition is not allowed.",
                            nname
                        );
                    }
                }
                Statement::Return(expr) => {
                    assert!(self.global == true, "cannot return {:?} from global scope", expr);
                    return self.eval_expr(expr);
                }
                Statement::FuncCall(name, args) => {
                    self.call_fn(name.as_str(), args);
                }
                Statement::BlockStmt(block) => {
                    self.global = false;
                    self.eval(block.body, None);
                    self.global = true;
                }
                Statement::VarAssign(nname, nexpr) => {
                    self.set_val(nname.as_str(), nexpr);
                }
                _ => unreachable!(),
            }
        }
        self.locals.clear();
        Value::Num(0f64)
    }

    pub fn get_val_from_var(&self, vname: &str) -> Option<Value> {
        if let Some(var) = self.vars.iter().filter(|var| var.name == vname).next() {
            Some(var.clone().val)
        } else if let Some(var) = self.locals.iter().filter(|var| var.name == vname).next() {
            Some(var.clone().val)
        } else {
            None
        }
    }

    pub fn call_fn(&mut self, name: &str, args: Vec<Expr>) -> Value {
        let func_struct = self
            .fns
            .iter()
            .find(|func| func.name == *name)
            .cloned()
            .unwrap();
        assert!(
            func_struct.args.len() == args.len(),
            "not enough / too many arguments provided. requested were {}, given were {}",
            func_struct.args.len(),
            args.len()
        );
        let arg_vals: Vec<EvalVariable> = args
            .iter()
            .zip(func_struct.args.iter())
            .map(|(value, name)| EvalVariable {
                val: self.eval_expr(value.clone()),
                name: name.to_owned(),
            })
            .collect();
        self.global = false;
        let res = self.eval(
            self.fns
                .iter()
                .filter(|func| func.name == *name)
                .next()
                .cloned()
                .unwrap()
                .body,
            Some(arg_vals),
        );
        self.global = true;
        res
    }

    pub fn set_val(&mut self, vname: &str, expr: Expr) {
        let expr = self.eval_expr(expr);
        if let Some(var) = self.vars.iter_mut().find(|var| var.name == vname) {
            var.val = expr;
        } else if let Some(var) = self.locals.iter_mut().find(|var| var.name == vname) {
            var.val = expr;
        } else {
            panic!("variable '{}' not found.", vname);
        }
    }

    pub fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Fn(name, args) => self.call_fn(name.as_str(), args),
            Expr::Ident(name) => self
                .get_val_from_var(name.as_str())
                .expect(format!("variable '{}' not found.", name).as_str()),
            Expr::Lit(val) => val.into(),
            _ => unreachable!(),
        }
    }
}
