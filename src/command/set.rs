use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::db::DB;
use crate::resp::Value as RespValue;

use super::Reply;

pub struct SetCommand;

impl SetCommand {
    fn parse_px(args: &Vec<RespValue>) -> Result<Option<Duration>> {
        if let (Some(RespValue::BulkString(px)), Some(RespValue::BulkString(millis))) =
            (args.get(2), args.get(3))
        {
            if px.to_lowercase() == "px" {
                let millis = millis.parse::<u64>()?;
                return Ok(Some(Duration::from_millis(millis)));
            }
        }

        Ok(None)
    }
}

impl Reply for SetCommand {
    fn reply(&self, args: Vec<RespValue>, db: Arc<Mutex<DB>>) -> RespValue {
        if let (Some(RespValue::BulkString(key)), Some(RespValue::BulkString(val))) =
            (args.get(0), args.get(1))
        {
            let px = match Self::parse_px(&args) {
                Ok(d) => d,
                Err(err) => return RespValue::Error(format!("failed to parse PX option {err}")),
            };
            db.lock().unwrap().set(key.clone(), val.clone(), px);
            RespValue::SimpleString("OK".to_string())
        } else {
            RespValue::Error("set requires key and value arguments".to_string())
        }
    }
}
