use std::collections::HashMap;

pub struct DB {
    data: HashMap<String, String>,
}

impl DB {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn get(&self, key: &String) -> Option<&String> {
        self.data.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key.clone(), value.clone());
    }
}
