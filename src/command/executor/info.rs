use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use std::error::Error;

pub struct info;

// ioredis's ready-check, sent right after handshake. parses
// redis_version/role out of this before it'll send real commands.
impl info {
    pub fn execute(_command_object: &Command, _context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        let body = "# Server\r\nredis_version:7.0.0\r\nredis_mode:standalone\r\n# Replication\r\nrole:master\r\n";
        Ok(format!("${}\r\n{}\r\n", body.len(), body).into_bytes())
    }
}
