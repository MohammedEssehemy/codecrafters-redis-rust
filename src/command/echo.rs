use std::sync::{Arc, Mutex};

use crate::db::DB;
use crate::resp::Value as RespValue;

use super::Reply;

pub struct EchoCommand;

impl Reply for EchoCommand {
    fn reply(&self, args: Vec<RespValue>, _db: Arc<Mutex<DB>>) -> RespValue {
        if args.len() == 1 {
            args.first().unwrap().clone()
        } else {
            RespValue::Error("echo requires exactly one argument".to_string())
        }
    }
}
