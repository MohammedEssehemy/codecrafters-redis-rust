use std::str::FromStr;
use std::sync::{Arc, Mutex};
use thiserror::Error;

use crate::db::DB;
use crate::resp::Value as RespValue;

use super::reply::Reply;

use super::echo::EchoCommand;
use super::get::GetCommand;
use super::ping::PingCommand;
use super::set::SetCommand;

pub enum Command {
    Ping(PingCommand),
    Echo(EchoCommand),
    Get(GetCommand),
    Set(SetCommand),
}

impl Command {
    pub fn reply(cmd: &str, args: Vec<RespValue>, db: Arc<Mutex<DB>>) -> RespValue {
        match cmd.parse::<Command>() {
            Err(err) => RespValue::Error(err.to_string()),
            Ok(command) => match command {
                Self::Ping(ping_command) => ping_command.reply(args, db),
                Self::Echo(echo_command) => echo_command.reply(args, db),
                Self::Get(get_command) => get_command.reply(args, db),
                Self::Set(set_command) => set_command.reply(args, db),
            },
        }
    }
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("command not found: ({0})")]
    InValidCommand(String),
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ping" => Ok(Command::Ping(PingCommand)),
            "echo" => Ok(Command::Echo(EchoCommand)),
            "get" => Ok(Command::Get(GetCommand)),
            "set" => Ok(Command::Set(SetCommand)),
            _ => Err(CommandError::InValidCommand(s.to_string())),
        }
    }
}
