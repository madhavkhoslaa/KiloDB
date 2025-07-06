use crate::command::command_enum::Command;
use crate::store::sorted_set_store::SortedSetStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::error::Error;

pub struct zrange;

impl commandExecutor for zrange {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::ZRANGE { key, start, stop } => {
                // Check if key exists in database
                if let Some(store) = context.DataBase.get(key) {
                    // Try to get the sorted set store
                    if let Some(sorted_set_store) = store.borrow().downcast_ref::<SortedSetStore>() {
                        // Key exists and is a sorted set, get range
                        let range = sorted_set_store.range(*start, *stop);
                        let mut response = format!("*{}\r\n", range.len()).into_bytes();
                        
                        for member in range {
                            response.extend_from_slice(&format!("${}\r\n{}\r\n", member.len(), member).into_bytes());
                        }
                        
                        Ok(response)
                    } else {
                        // Key exists but is not a sorted set, return error
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