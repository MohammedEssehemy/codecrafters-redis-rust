use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
struct DBVal {
    value: String,
    expire_at: Option<Instant>,
}

pub struct DB {
    data: HashMap<String, DBVal>,
}

impl DB {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &String) -> Option<String> {
        match self.data.get(key) {
            None => None,
            Some(v) => {
                if let Some(expire_at) = v.expire_at {
                    if Instant::now() > expire_at {
                        self.del(key);
                        return None;
                    }
                }
                Some(v.value.clone())
            }
        }
    }

    pub fn set(&mut self, key: String, value: String, expiry: Option<Duration>) {
        let expire_at = expiry.map(|d| Instant::now() + d);
        self.data.insert(key, DBVal { value, expire_at });
    }

    pub fn del(&mut self, key: &String) {
        self.data.remove(key);
    }
}
