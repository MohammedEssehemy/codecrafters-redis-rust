use std::sync::{Arc, Mutex};

use crate::db::DB;
use crate::resp::Value as RespValue;

use super::Reply;

pub struct SetCommand;

impl Reply for SetCommand {
    fn reply(&self, args: Vec<RespValue>, db: Arc<Mutex<DB>>) -> RespValue {
        if let (
            //
            Some(RespValue::BulkString(key)),
            Some(RespValue::BulkString(val)),
        ) = (args.get(0), args.get(1))
        {
            db.lock().unwrap().set(key.clone(), val.clone());
            RespValue::SimpleString("OK".to_string())
        } else {
            RespValue::Error(format!("set requires key and value arguments"))
        }
    }
}
