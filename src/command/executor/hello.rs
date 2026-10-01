use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use std::error::Error;

pub struct hello;

// we only speak RESP2, so always reply proto=2 regardless of what the
// client asked for. same shape real Redis uses for its RESP2 HELLO reply.
// without this ioredis's handshake errors out and it drops the connection.
impl hello {
    pub fn execute(_command_object: &Command, _context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        let fields: Vec<(&str, String)> = vec![
            ("server", "redis".to_string()),
            ("version", "7.0.0".to_string()),
            ("proto", "2".to_string()),
            ("id", "1".to_string()),
            ("mode", "standalone".to_string()),
            ("role", "master".to_string()),
        ];

        let mut out = format!("*{}\r\n", fields.len() * 2 + 2);
        for (key, value) in &fields {
            out.push_str(&format!("${}\r\n{}\r\n", key.len(), key));
            if key == &"proto" || key == &"id" {
                out.push_str(&format!(":{}\r\n", value));
            } else {
                out.push_str(&format!("${}\r\n{}\r\n", value.len(), value));
            }
        }
        // modules: empty array
        out.push_str("$7\r\nmodules\r\n*0\r\n");

        Ok(out.into_bytes())
    }
}
