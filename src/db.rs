use std::collections::HashMap;
use std::fmt::Display;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct NodeAddress {
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub enum NodeRole {
    Master,
    Slave(NodeAddress),
}

impl Display for NodeRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeRole::Master => write!(f, "master"),
            NodeRole::Slave(_) => write!(f, "slave"),
        }
    }
}

#[derive(Clone, Debug)]
struct DBVal {
    value: String,
    expire_at: Option<Instant>,
}

pub struct DB {
    pub role: NodeRole,
    pub replid: String,
    pub repl_offset: i32,
    data: HashMap<String, DBVal>,
}

impl DB {
    pub fn new(role: NodeRole) -> Self {
        Self {
            role,
            replid: "8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb".to_string(),
            repl_offset: 0,
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
