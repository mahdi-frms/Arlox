use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{ast::AstNodeRef, interpret::Value};
type NativeImpl = fn(Vec<Value>) -> Result<Value, ()>;

#[derive(Clone)]
pub enum Implementation {
    NativeImpl(NativeImpl),
    LoxImpl(AstNodeRef),
}

impl PartialEq for Implementation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NativeImpl(l0), Self::NativeImpl(r0)) => l0 == r0,
            (Self::LoxImpl(l0), Self::LoxImpl(r0)) => Arc::ptr_eq(l0, r0),
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Function {
    params: Vec<String>,
    code: Implementation,
}

impl Function {
    pub fn code(&self) -> &Implementation {
        &self.code
    }
    pub fn params(&self) -> &Vec<String> {
        &self.params
    }
    pub fn create(code: Implementation, params: Vec<String>) -> Function {
        Function { code, params }
    }
}

fn log(args: Vec<Value>) -> Result<Value, ()> {
    for a in args {
        print!("{} ", a);
    }
    println!();
    Ok(Value::Nil)
}

fn clock(_: Vec<Value>) -> Result<Value, ()> {
    Ok(Value::Number(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as f64,
    ))
}

pub fn all_natives() -> Vec<(String, Function)> {
    let mut all = vec![];

    all.push((
        "log".to_string(),
        Function::create(Implementation::NativeImpl(log), vec!["".to_string()]),
    ));
    all.push((
        "clock".to_string(),
        Function::create(Implementation::NativeImpl(clock), vec![]),
    ));

    all
}
