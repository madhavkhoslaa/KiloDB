use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use std::error::Error;

pub struct client;

// ioredis fires CLIENT SETINFO LIB-NAME/LIB-VER after handshake, doesn't
// care about the reply content, just needs something well-formed back.
impl client {
    pub fn execute(_command_object: &Command, _context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(b"+OK\r\n".to_vec())
    }
}
