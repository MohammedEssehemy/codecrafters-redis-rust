use std::sync::{Arc, Mutex};

use crate::db::DB;
use crate::resp::Value as RespValue;

use super::Reply;

pub struct PingCommand;

impl Reply for PingCommand {
    fn reply(&self, _args: Vec<RespValue>, _db: Arc<Mutex<DB>>) -> RespValue {
        RespValue::SimpleString("PONG".to_string())
    }
}
