use crate::{
    ast::{AssignExpr, Block, BreakStmt, FunCall, IfStmt, WhileStmt},
    lox_error,
    token::{Token, TokenKind},
};
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
    #[allow(dead_code)]
    NativeFun(fn(Vec<Value>) -> Result<Value, ()>),
    Nil,
}

struct Environment {
    maps: Vec<HashMap<String, Value>>,
}

impl Environment {
    fn new() -> Environment {
        Environment {
            maps: vec![HashMap::new()],
        }
    }
    fn get(&mut self, name: &String) -> Option<&mut Value> {
        for m in self.maps.iter_mut().rev() {
            if let Some(v) = m.get_mut(name) {
                return Some(v);
            }
        }
        return None;
    }
    fn set(&mut self, name: &String, value: Value) {
        if let Some(m) = self.maps.last_mut() {
            m.insert(name.clone(), value);
        }
    }
    fn assign(&mut self, name: &String, value: Value) {
        if let Some(v) = self.get(name) {
            *v = value;
        } else {
            self.set(name, value);
        }
    }
    fn init(&mut self, name: &String, value: Value) {
        self.set(name, value);
    }
    fn enter(&mut self) {
        self.maps.push(HashMap::new())
    }
    fn exit(&mut self) {
        self.maps.pop();
    }
}

pub struct Interpretor {
    env: Environment,
    breaking: Option<Token>,
}

pub fn interpret(ast: Ast) -> Option<Value> {
    let mut interpretor = Interpretor::new();
    let rsl = ast.root().interpret(&mut interpretor).ok();
    if let Some(tkn) = interpretor.breaking {
        lox_error(tkn.line(), "break statement out ot loop");
        None
    } else {
        rsl
    }
}

impl Interpretor {
    fn new() -> Interpretor {
        Interpretor {
            env: Environment::new(),
            breaking: None,
        }
    }
    pub fn interpret_literal(&mut self, node: &LiteralExpr) -> Result<Value, ()> {
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
                None => {
                    lox_error(
                        node.token().line(),
                        format!("undefind variable '{}'", node.token().text()).as_str(),
                    );
                    Err(())
                }
            },
            _ => Err(()),
        }
    }
    pub fn interpret_group(&mut self, node: &GroupExpr) -> Result<Value, ()> {
        node.expr().interpret(self)
    }
    pub fn interpret_assignment(&mut self, node: &AssignExpr) -> Result<Value, ()> {
        let value = node.expr().interpret(self)?;
        self.env.assign(node.variable().text(), value.clone());
        Ok(value)
    }
    pub fn interpret_if_stmt(&mut self, node: &IfStmt) -> Result<Value, ()> {
        let condition = node.expr().interpret(self)?;
        if condition.truth() {
            node.stmt().interpret(self)?;
        } else {
            if let Some(elstmt) = node.elstmt() {
                elstmt.interpret(self)?;
            }
        }
        Ok(Value::Nil)
    }
    pub fn interpret_while_stmt(&mut self, node: &WhileStmt) -> Result<Value, ()> {
        while node.expr().interpret(self)?.truth() {
            node.stmt().interpret(self)?;
            if let Some(_) = self.breaking {
                self.breaking = None;
                break;
            }
        }
        Ok(Value::Nil)
    }
    pub fn interpret_break_stmt(&mut self, node: &BreakStmt) -> Result<Value, ()> {
        self.breaking = Some(node.token().clone());
        Ok(Value::Nil)
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
    pub fn interpret_and(&mut self, node: &BinaryExpr) -> Result<Value, ()> {
        let left = node.lexpr().interpret(self)?;
        if !left.truth() {
            return Ok(Value::Boolean(false));
        }
        node.rexpr().interpret(self)
    }
    pub fn interpret_or(&mut self, node: &BinaryExpr) -> Result<Value, ()> {
        let left = node.lexpr().interpret(self)?;
        if left.truth() {
            return Ok(Value::Boolean(true));
        }
        node.rexpr().interpret(self)
    }
    pub fn interpret_binary(&mut self, node: &BinaryExpr) -> Result<Value, ()> {
        match node.token().kind() {
            TokenKind::EqualEqual => Ok(Value::Boolean(
                node.lexpr().interpret(self) == node.rexpr().interpret(self),
            )),
            TokenKind::BangEqual => Ok(Value::Boolean(
                node.lexpr().interpret(self) != node.rexpr().interpret(self),
            )),
            TokenKind::Or => self.interpret_or(node),
            TokenKind::And => self.interpret_and(node),
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
        self.env.init(node.name().text(), value);

        Ok(Value::Nil)
    }
    pub fn interpret_fun_call(&mut self, node: &FunCall) -> Result<Value, ()> {
        let line = node.line();
        let callee = node.callee().interpret(self)?;
        let callee = match callee {
            Value::NativeFun(f) => f,
            _ => {
                lox_error(line, format!("{} is not callable", callee).as_str());
                return Err(());
            }
        };
        let mut args = vec![];
        for a in node.args() {
            args.push(a.interpret(self)?);
        }
        Ok(callee(args)?)
    }
    pub fn interpret_program(&mut self, node: &Program) -> Result<Value, ()> {
        for s in node.decs() {
            s.interpret(self)?;
            if let Some(_) = self.breaking {
                break;
            }
        }
        Ok(Value::Nil)
    }
    pub fn interpret_block(&mut self, node: &Block) -> Result<Value, ()> {
        self.env.enter();
        for s in node.decs() {
            s.interpret(self)?;
            if let Some(_) = self.breaking {
                break;
            }
        }
        self.env.exit();
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
            Value::NativeFun(_) => "[Native Function]".to_string(),
        };
        write!(f, "{}", rep)
    }
}
