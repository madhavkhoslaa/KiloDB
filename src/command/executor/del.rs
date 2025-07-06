use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use std::error::Error;

pub struct del;

impl commandExecutor for del {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::DEL { keys } => {
                for key in keys {
                    context.DataBase.store.remove(key.as_str());
                    // Note: TTL removal would need to be implemented based on your TTL store structure
                }
                Ok(b"+OK\r\n".to_vec())
            }
            _ => Ok(b"-ERR wrong command\r\n".to_vec()),
        }
    }
}
