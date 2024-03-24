use std::sync::{Arc, Mutex};

use crate::db::DB;
use crate::resp::Value as RespValue;

use super::Reply;

pub struct InfoCommand;

impl Reply for InfoCommand {
    fn reply(&self, _args: Vec<RespValue>, _db: Arc<Mutex<DB>>) -> RespValue {
        RespValue::BulkString(String::from(
            "# Replication
role:master
connected_slaves:0
",
        ))
    }
}
