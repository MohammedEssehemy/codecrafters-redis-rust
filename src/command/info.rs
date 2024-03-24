use std::sync::{Arc, Mutex};

use crate::db::DB;
use crate::resp::Value as RespValue;

use super::Reply;

pub struct InfoCommand;

impl Reply for InfoCommand {
    fn reply(&self, _args: Vec<RespValue>, _db: Arc<Mutex<DB>>) -> RespValue {
        let role = _db.lock().unwrap().role.to_string();
        RespValue::BulkString(
            format!(
                "# Replication
role:{role}
connected_slaves:0
"
            )
            .to_string(),
        )
    }
}
