use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use std::error::Error;

pub struct echo;

impl commandExecutor for echo {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::ECHO { message } => {
                // Return the message in RESP bulk string format
                let response = format!("${}\r\n{}\r\n", message.len(), message);
                Ok(response.into_bytes())
            }
            _ => {
                // This should never happen since we only match ECHO
                Ok(b"-ERR unexpected command\r\n".to_vec())
            }
        }
    }
}
