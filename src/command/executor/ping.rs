use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use std::error::Error;

pub struct ping;

// not commandExecutor, just &context - doesn't touch the store so it
// can run under a read lock
impl ping {
    pub fn execute(commandObject: &Command, _context: &context) -> Result<Vec<u8>, Box<dyn Error>> {
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
