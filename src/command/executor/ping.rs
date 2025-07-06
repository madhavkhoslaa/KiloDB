use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use std::error::Error;

pub struct ping;

impl commandExecutor for ping {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::PING => {
                // Return PONG in RESP simple string format
                Ok(b"+PONG\r\n".to_vec())
            }
            _ => {
                // This should never happen since we only match PING
                Ok(b"-ERR unexpected command\r\n".to_vec())
            }
        }
    }
}
