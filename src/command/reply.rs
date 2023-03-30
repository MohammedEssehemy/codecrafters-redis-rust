use std::sync::{Arc, Mutex};

use crate::db::DB;
use crate::resp::Value as RespValue;

pub trait Reply {
    fn reply(&self, args: Vec<RespValue>, db: Arc<Mutex<DB>>) -> RespValue;
}
