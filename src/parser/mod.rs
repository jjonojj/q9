#![allow(dead_code)]

use crate::lexer::*;

pub mod expr;
use expr::*;

#[derive(Debug, Clone)]
pub enum Statement {
    FuncCall(String, Vec<Expr>),
    Return(Expr),
    FuncDef(Function, Vec<String>),
    VarDef(String, Expr),
    VarAssign(String, Expr),
    BlockStmt(Block),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub content: Vec<Statement>,
}

//

pub struct Parser {
    lexer: Tokenizer,
    cur: Token,
    peek: Token,
    global: bool,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut parser = Self {
            lexer: Tokenizer::new(input),
            cur: Token::None,
            peek: Token::None,
            global: true,
        };
        parser.advance();
        parser.advance();
        parser
    }

    fn advance(&mut self) -> Token {
        let prev = self.cur.clone();
        self.cur = self.peek.clone();
        self.peek = self.lexer.next();
        prev
    }

    fn check(&self, expected: Token) -> Result<(), String> {
        match &self.cur {
            cur if *cur == expected => Ok(()),
            _ => Err(format!("expected {expected}, found {}", self.cur)),
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        self.advance();
        self.check(expected)
    }

    fn parse_return(&mut self) -> Result<Statement, String> {
        self.advance();
        Ok(Statement::Return(self.parse_expr()?))
    }

    fn parse_func_def(&mut self) -> Result<Statement, String> {
        self.global = false;
        self.advance();
        let name = match &self.cur {
            Token::Str(str) => str.clone(),
            _ => return Err(format!("expected function name, found {}", self.cur)),
        };

        self.expect(Token::LParen)?;
        self.advance();
        let mut args = Vec::new();
        while self.cur != Token::RParen {
            args.push(self.parse_ident()?);
            if self.cur == Token::RParen { break };
            self.check(Token::Comma)?;
            self.advance();
        }
        self.check(Token::RParen)?;
        self.advance();
        let body = self.parse_block()?;
        self.global = true;
        Ok(Statement::FuncDef(Function { name, body }, args))
    }

    fn parse_var_def(&mut self) -> Result<Statement, String> {
        self.advance(); // consume let
        let (name, expr) = match self.parse_var_assign() {
            Ok(Statement::VarAssign(name, expr)) => (name, expr),
            Err(err) => return Err(err),
            _ => return Err("unreachable".to_owned()),
        };

        Ok(Statement::VarDef(name, expr))
    }

    fn parse_var_assign(&mut self) -> Result<Statement, String> {
        let name = self.parse_ident()?;
        self.advance();
        let expr = self.parse_expr()?;
        Ok(Statement::VarAssign(name, expr))
    }
    
    fn parse_func_call(&mut self) -> Result<Statement, String> {
        let name = self.parse_ident()?;
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
        Ok(Statement::FuncCall(name, args))
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        // println!("parsing... > cur: {}, peek: {}, global: {}", self.cur, self.peek, self.global);
        match &self.cur {
            Token::Return => {
                return self.parse_return();
            }
            Token::Fn => {
                if self.global {
                    return self.parse_func_def();
                } else {
                    return Err(format!("cannot define functions in functions"));
                }
            }
            Token::Let => {
                return self.parse_var_def();
            }
            Token::LBrace => {
                return Ok(Statement::BlockStmt( self.parse_block()? ))
            }
            _ => {
                return match self.peek {
                    Token::Eq => self.parse_var_assign(),
                    Token::LParen => self.parse_func_call(),
                    _ => return Err(format!("unexpected token: {}", self.cur)),
                };
            }
        };
    }

    fn parse_ident(&mut self) -> Result<String, String> {
        let val = match &self.cur {
            Token::Str(val) => Ok(val.clone()),
            _ => Err(format!("expected string literal, found {}", self.cur)),
        };
        self.advance();
        val
    }

    fn parse_num(&mut self) -> Result<String, String> {
        let val = match &self.cur {
            Token::Num(val) => Ok(val.clone()),
            _ => Err(format!("expected string literal, found {}", self.cur)),
        };
        self.advance();
        val
    }
    
    fn parse_block(&mut self) -> Result<Block, String> {
        self.check(Token::LBrace)?;
        let mut block = Block { body: Vec::new() };
        self.advance();

        while self.cur != Token::RBrace {
            block.body.push(self.parse_statement()?);
        }

        self.check(Token::RBrace)?;
        self.advance();
        Ok(block)
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut prog = Program {
            content: Vec::new(),
        };

        while self.cur != Token::EOF {
            prog.content.push(self.parse_statement()?);
        }

        Ok(prog)
    }
}
