use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use std::error::Error;

pub struct flushdb;

impl commandExecutor for flushdb {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::FLUSHDB => {
                // Clear the main database store
                context.DataBase.store.clear();
                
                // Clear the TTL store
                context.TTLStore.store.clear();
                
                // Return OK response in RESP format
                Ok(b"+OK\r\n".to_vec())
            }
            _ => {
                // This should never happen since we only match FLUSHDB
                Ok(b"-ERR unexpected command\r\n".to_vec())
            }
        }
    }
}
