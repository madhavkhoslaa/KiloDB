use crate::command::command_enum::Command;
use crate::store::hash_store::HashStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::error::Error;

pub struct hgetall;

impl commandExecutor for hgetall {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::HGETALL { key } => {
                // Check if key exists in database
                if let Some(store) = context.DataBase.get(key) {
                    // Try to get the hash store
                    if let Some(hash_store) = store.borrow().downcast_ref::<HashStore>() {
                        // Key exists and is a hash, get all fields
                        let data = hash_store.get_all();
                        let mut response = format!("*{}\r\n", data.len() * 2).into_bytes();
                        
                        for (field, value) in data {
                            // Add field
                            response.extend_from_slice(&format!("${}\r\n{}\r\n", field.len(), field).into_bytes());
                            // Add value
                            response.extend_from_slice(&format!("${}\r\n{}\r\n", value.len(), value).into_bytes());
                        }
                        
                        Ok(response)
                    } else {
                        // Key exists but is not a hash, return error
                        Ok(b"-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                    }
                } else {
                    // Key doesn't exist, return empty array
                    Ok(b"*0\r\n".to_vec())
                }
            }
            _ => {
                Ok(b"-ERR unexpected command\r\n".to_vec())
            }
        }
    }
} 