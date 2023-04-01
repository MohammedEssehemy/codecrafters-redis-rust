use std::sync::{Arc, Mutex};

use crate::db::DB;
use crate::resp::Value as RespValue;

use super::Reply;

pub struct GetCommand;

impl Reply for GetCommand {
    fn reply(&self, args: Vec<RespValue>, db: Arc<Mutex<DB>>) -> RespValue {
        if let Some(RespValue::BulkString(key)) = args.get(0) {
            db.lock()
                .unwrap()
                .get(key)
                .map(|s| RespValue::SimpleString(s))
                .unwrap_or(RespValue::Null)
        } else {
            RespValue::Error(format!("get requires key argument"))
        }
    }
}
