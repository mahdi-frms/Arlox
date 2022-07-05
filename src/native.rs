use std::time::{SystemTime, UNIX_EPOCH};

use crate::interpret::Value;

#[derive(PartialEq, Clone)]
pub struct NativeFunction {
    name: String,
    param_count: isize,
    func: fn(Vec<Value>) -> Result<Value, ()>,
}

impl NativeFunction {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn func(&self) -> fn(Vec<Value>) -> Result<Value, ()> {
        self.func
    }
    pub fn param_count(&self) -> isize {
        self.param_count
    }
    fn create(
        name: String,
        func: fn(Vec<Value>) -> Result<Value, ()>,
        param_count: isize,
    ) -> NativeFunction {
        NativeFunction {
            name,
            func,
            param_count,
        }
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

pub fn all_natives() -> Vec<NativeFunction> {
    let mut all = vec![];

    all.push(NativeFunction::create("log".to_string(), log, -1));
    all.push(NativeFunction::create("clock".to_string(), clock, 0));

    all
}
