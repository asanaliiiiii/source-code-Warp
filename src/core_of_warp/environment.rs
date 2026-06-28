use std::collections::HashMap;

pub struct Environment {
    scopes: HashMap<String, String>,
}

impl Environment {
    pub fn new() -> Self { Self { scopes: HashMap::new() } }
    pub fn set(&mut self, name: String, value: String) { self.scopes.insert(name, value); }
    pub fn get(&self, name: &str) -> Option<String> { self.scopes.get(name).cloned() }
    pub fn get_all(&self) -> &HashMap<String, String> { &self.scopes }
}