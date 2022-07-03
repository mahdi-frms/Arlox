use crate::token::TokenKind;
use std::collections::hash_map::HashMap;
use std::fmt::Display;

use crate::ast::{
    Ast, BinaryExpr, ExprStmt, GroupExpr, LiteralExpr, PrintStmt, Program, UnaryExpr, VarDecl,
};

#[derive(PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct Interpretor {
    env: HashMap<String, Value>,
}

pub fn interpret(ast: Ast) -> Option<Value> {
    let mut interpretor = Interpretor::new();
    ast.root().interpret(&mut interpretor).ok()
}

impl Interpretor {
    pub fn new() -> Interpretor {
        Interpretor {
            env: HashMap::new(),
        }
    }
    pub fn interpret_literal(&self, node: &LiteralExpr) -> Result<Value, ()> {
        match node.token().kind() {
            TokenKind::Nil => Ok(Value::Nil),
            TokenKind::Number => match node.token().text().parse::<f64>() {
                Ok(num) => Ok(Value::Number(num)),
                Err(_) => {
                    crate::lox_error(
                        node.token().line(),
                        &format!("invalid number ({})", node.token().text()),
                    );
                    Err(())
                }
            },
            TokenKind::String => Ok(Value::String(
                node.token().text()[1..node.token().text().len() - 1].to_owned(),
            )),
            TokenKind::True => Ok(Value::Boolean(true)),
            TokenKind::False => Ok(Value::Boolean(false)),
            TokenKind::Identifier => match self.env.get(node.token().text()) {
                Some(v) => Ok(v.clone()),
                None => Err(()),
            },
            _ => Err(()),
        }
    }
    pub fn interpret_group(&mut self, node: &GroupExpr) -> Result<Value, ()> {
        node.expr().interpret(self)
    }
    pub fn interpret_unary(&mut self, node: &UnaryExpr) -> Result<Value, ()> {
        if node.token().kind() == TokenKind::Bang {
            Ok(Value::Boolean(!node.expr().interpret(self)?.truth()))
        } else {
            match node.expr().interpret(self)? {
                Value::Number(num) => Ok(Value::Number(-num)),
                _ => {
                    crate::lox_error(node.token().line(), "expected number after '-'");
                    Err(())
                }
            }
        }
    }
    pub fn interpret_plus(&mut self, node: &BinaryExpr) -> Result<Value, ()> {
        match (node.lexpr().interpret(self)?, node.rexpr().interpret(self)?) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => {
                crate::lox_error(
                    node.token().line(),
                    "operator '+' can only be used on number or string types",
                );
                Err(())
            }
        }
    }
    pub fn interpret_math(&mut self, node: &BinaryExpr) -> Result<Value, ()> {
        match (node.lexpr().interpret(self)?, node.rexpr().interpret(self)?) {
            (Value::Number(a), Value::Number(b)) => match node.token().kind() {
                TokenKind::Star => Ok(Value::Number(a * b)),
                TokenKind::Slash => Ok(Value::Number(a / b)),
                TokenKind::Minus => Ok(Value::Number(a - b)),
                TokenKind::GreaterEqual => Ok(Value::Boolean(a >= b)),
                TokenKind::LessEqual => Ok(Value::Boolean(a <= b)),
                TokenKind::Less => Ok(Value::Boolean(a < b)),
                TokenKind::Greater => Ok(Value::Boolean(a > b)),
                _ => Err(()),
            },
            _ => {
                crate::lox_error(
                    node.token().line(),
                    "operator '+' can only be used on number or string types",
                );
                Err(())
            }
        }
    }
    pub fn interpret_binary(&mut self, node: &BinaryExpr) -> Result<Value, ()> {
        match node.token().kind() {
            TokenKind::EqualEqual => Ok(Value::Boolean(
                node.lexpr().interpret(self) == node.rexpr().interpret(self),
            )),
            TokenKind::BangEqual => Ok(Value::Boolean(
                node.lexpr().interpret(self) != node.rexpr().interpret(self),
            )),
            TokenKind::Plus => self.interpret_plus(node),
            _ => self.interpret_math(node),
        }
    }
    pub fn interpret_print_stmt(&mut self, node: &PrintStmt) -> Result<Value, ()> {
        let value = node.expr().interpret(self)?;
        println!("{}", value);
        Ok(Value::Nil)
    }
    pub fn interpret_expr_stmt(&mut self, node: &ExprStmt) -> Result<Value, ()> {
        node.expr().interpret(self)?;
        Ok(Value::Nil)
    }
    pub fn interpret_var_decl(&mut self, node: &VarDecl) -> Result<Value, ()> {
        let value = match node.expr() {
            Some(e) => e.interpret(self)?,
            None => Value::Nil,
        };
        self.env.insert(node.name().text().clone(), value);

        Ok(Value::Nil)
    }
    pub fn interpret_program(&mut self, node: &Program) -> Result<Value, ()> {
        for s in node.stmts() {
            s.interpret(self)?;
        }
        Ok(Value::Nil)
    }
}

impl Value {
    fn truth(&self) -> bool {
        if *self == Value::Nil || *self == Value::Boolean(false) {
            false
        } else {
            true
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rep = match self {
            Value::Number(num) => num.to_string(),
            Value::String(s) => s.clone(),
            Value::Nil => String::from("NIL"),
            Value::Boolean(b) => b.to_string(),
        };
        write!(f, "{}", rep)
    }
}
