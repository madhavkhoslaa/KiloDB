use crate::command::command_enum::Command;
use crate::store::hash_store::HashStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::error::Error;

pub struct hexists;

impl commandExecutor for hexists {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::HEXISTS { key, field } => {
                // Check if key exists in database
                if let Some(store) = context.DataBase.get(key) {
                    // Try to get the hash store
                    if let Some(hash_store) = store.borrow().downcast_ref::<HashStore>() {
                        // Key exists and is a hash, check if field exists
                        let exists = hash_store.exists(field);
                        Ok(format!(":{}\r\n", if exists { 1 } else { 0 }).into_bytes())
                    } else {
                        // Key exists but is not a hash, return error
                        Ok(b"-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                    }
                } else {
                    // Key doesn't exist, return 0
                    Ok(b":0\r\n".to_vec())
                }
            }
            _ => {
                Ok(b"-ERR unexpected command\r\n".to_vec())
            }
        }
    }
} 