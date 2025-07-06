use crate::command::command_enum::Command;
use crate::store::hash_store::HashStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::error::Error;

pub struct hget;

impl commandExecutor for hget {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::HGET { key, field } => {
                // Check if key exists in database
                if let Some(store) = context.DataBase.get(key) {
                    // Try to get the hash store
                    if let Some(hash_store) = store.borrow().downcast_ref::<HashStore>() {
                        // Key exists and is a hash, get the field
                        if let Some(value) = hash_store.get(field) {
                            // Return the value in RESP bulk string format
                            let response = format!("${}\r\n{}\r\n", value.len(), value);
                            Ok(response.into_bytes())
                        } else {
                            // Field doesn't exist, return null
                            Ok(b"$-1\r\n".to_vec())
                        }
                    } else {
                        // Key exists but is not a hash, return error
                        Ok(b"-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                    }
                } else {
                    // Key doesn't exist, return null
                    Ok(b"$-1\r\n".to_vec())
                }
            }
            _ => {
                Ok(b"-ERR unexpected command\r\n".to_vec())
            }
        }
    }
} 