use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use std::error::Error;

pub struct echo;

// not commandExecutor, just &context - doesn't touch the store so it
// can run under a read lock
impl echo {
    pub fn execute(commandObject: &Command, _context: &context) -> Result<Vec<u8>, Box<dyn Error>> {
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
