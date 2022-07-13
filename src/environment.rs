use crate::interpret::Value;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;
use std::sync::Arc;
pub type Env = Arc<RefCell<Environment>>;

#[derive(Default)]
pub struct Environment {
    map: HashMap<String, Value>,
    parent: Option<Env>,
}

impl Environment {
    pub fn new(par: Option<Env>) -> Env {
        let mut env = Environment::default();
        env.parent = par;
        Arc::new(RefCell::new(env))
    }
    pub fn get(&self, name: &String) -> Option<Value> {
        if let Some(v) = self.map.get(name) {
            Some(v.clone())
        } else if let Some(p) = self.parent.clone() {
            p.borrow_mut().get(name)
        } else {
            None
        }
    }
    pub fn depth(&self, name: &String) -> Option<usize> {
        if self.map.contains_key(name) {
            Some(0)
        } else if let Some(p) = self.parent.clone() {
            p.borrow_mut().depth(name).map(|d| d + 1)
        } else {
            None
        }
    }
    pub fn assign(&mut self, name: String, value: Value) -> bool {
        if let Some(v) = self.map.get_mut(&name) {
            *v = value;
            true
        } else if let Some(p) = self.parent.clone() {
            p.borrow_mut().assign(name, value)
        } else {
            false
        }
    }
    pub fn init(&mut self, name: String, value: Value) {
        self.map.insert(name, value);
    }

    pub fn parent(&self) -> Option<Arc<RefCell<Environment>>> {
        self.parent.clone()
    }
}
