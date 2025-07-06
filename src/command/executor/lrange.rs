use crate::command::command_enum::Command;
use crate::store::list_store::ListStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::error::Error;

pub struct lrange;

impl commandExecutor for lrange {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::LRANGE { key, start, stop } => {
                // Check if key exists in database
                if let Some(store) = context.DataBase.get(key) {
                    // Try to get the list store
                    if let Some(list_store) = store.borrow().downcast_ref::<ListStore>() {
                        // Key exists and is a list, get range
                        let range = list_store.range(*start, *stop);
                        let mut response = format!("*{}\r\n", range.len()).into_bytes();
                        
                        for value in range {
                            response.extend_from_slice(&format!("${}\r\n{}\r\n", value.len(), value).into_bytes());
                        }
                        
                        Ok(response)
                    } else {
                        // Key exists but is not a list, return error
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