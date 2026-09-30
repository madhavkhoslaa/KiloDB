use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use std::error::Error;

pub struct dbsize;

// not commandExecutor, just &context - only reads, so it can run
// under a read lock
impl dbsize {
    pub fn execute(commandObject: &Command, context: &context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::DBSIZE => {
                // Get the number of keys in the database
                let key_count = context.DataBase.store.len();
                
                // Return the count in RESP integer format
                let response = format!(":{}\r\n", key_count);
                Ok(response.into_bytes())
            }
            _ => {
                // This should never happen since we only match DBSIZE
                Ok(b"-ERR unexpected command\r\n".to_vec())
            }
        }
    }
}
