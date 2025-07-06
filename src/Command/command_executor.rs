use crate::{store_containers::core_context::context, Command::command_enum::Command};
use crate::executors::kv_store::kv_store;
use std::error::Error;
pub struct command_executor {}
impl command_executor {
    pub fn execute_command(
        command: &Command,
        context: &mut context,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        match command {
            Command::SET { key: _key, value: _value, ttl: _ttl } => {
                kv_store::set(command, context)
            }
            Command::GET { key: _key } => {
                kv_store::get(command, context)
            }
            Command::DEL { keys: _keys } => {
                kv_store::del(command, context)
            }
            _ => Ok(b"$-1\r\n".to_vec()),
        }
    }
}
