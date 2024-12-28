use crate::value::Value;
use std::collections::HashMap;

pub struct Environment {
    variables: HashMap<String, Value>,
}
impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set_var(&mut self, name: &String, val: Value) {
        self.variables.insert(name.to_string(), val);
    }

    pub fn get_var(&mut self, name: &String) -> Option<&Value> {
        self.variables.get(name)
    }
}
