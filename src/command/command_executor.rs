use crate::command::executor::{del, get, set};
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
            } => set::set::execute(command.clone(), context),
            Command::GET { key: _key } => get::get::execute(command.clone(), context),
            Command::DEL { keys: _keys } => del::del::execute(command.clone(), context),
            _ => Ok(b"$-1\r\n".to_vec()),
        }
    }
}
