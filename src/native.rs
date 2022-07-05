use crate::interpret::Value;

#[derive(PartialEq, Clone)]
pub struct NativeFunction {
    name: String,
    func: fn(Vec<Value>) -> Result<Value, ()>,
}

impl NativeFunction {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn func(&self) -> fn(Vec<Value>) -> Result<Value, ()> {
        self.func
    }
    fn create(name: String, func: fn(Vec<Value>) -> Result<Value, ()>) -> NativeFunction {
        NativeFunction { name, func }
    }
}

fn log(args: Vec<Value>) -> Result<Value, ()> {
    for a in args {
        print!("{} ", a);
    }
    println!();
    Ok(Value::Nil)
}

pub fn all_natives() -> Vec<NativeFunction> {
    let mut all = vec![];

    all.push(NativeFunction::create("log".to_string(), log));

    all
}
