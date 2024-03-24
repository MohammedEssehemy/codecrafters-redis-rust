use std::sync::{Arc, Mutex};

use crate::db::DB;
use crate::resp::Value as RespValue;

use super::Reply;

pub struct InfoCommand;

impl Reply for InfoCommand {
    fn reply(&self, _args: Vec<RespValue>, _db: Arc<Mutex<DB>>) -> RespValue {
        let db = _db.lock().unwrap();
        RespValue::BulkString(
            format!(
                "# Replication
role:{0}
connected_slaves:0
master_replid:{1}
master_repl_offset:{2}
",
                &db.role, &db.replid, &db.repl_offset
            )
            .to_string(),
        )
    }
}
