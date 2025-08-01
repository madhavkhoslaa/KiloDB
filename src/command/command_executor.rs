use crate::command::executor::{del, dbsize, echo, exists, expire, flushdb, get, ping, set};
use crate::traits::command::commandExecutor;
use crate::{command::command_enum::Command, store_containers::core_context::context};
use std::error::Error;
pub struct command_executor {}

impl command_executor {
    pub fn execute_command(
        command: &Command,
        context: &mut context,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        match command {
            Command::SET {
                key: _key,
                value: _value,
                ttl: _ttl,
            } => set::set::execute(command, context),
            Command::GET { key: _key } => get::get::execute(command, context),
            Command::DEL { keys: _keys } => del::del::execute(command, context),
            Command::EXISTS { keys: _keys } => exists::exists::execute(command, context),
            Command::EXPIRE {
                key: _key,
                seconds: _seconds,
            } => expire::expire::execute(command, context),
            Command::FLUSHDB => flushdb::flushdb::execute(command, context),
            Command::DBSIZE => dbsize::dbsize::execute(command, context),
            Command::ECHO { message: _message } => echo::echo::execute(command, context),
            Command::PING => ping::ping::execute(command, context),
            _ => Ok(b"$-1\r\n".to_vec()),
        }
    }
}
