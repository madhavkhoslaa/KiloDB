use crate::command::command_enum::Command;
use crate::store::set_store::SetStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::error::Error;

pub struct smembers;

impl commandExecutor for smembers {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::SMEMBERS { key } => {
                // Check if key exists in database
                if let Some(store) = context.DataBase.get(key) {
                    // Try to get the set store
                    if let Some(set_store) = store.borrow().downcast_ref::<SetStore>() {
                        // Key exists and is a set, get all members
                        let members = set_store.members();
                        let mut response = format!("*{}\r\n", members.len()).into_bytes();
                        
                        for member in members {
                            response.extend_from_slice(&format!("${}\r\n{}\r\n", member.len(), member).into_bytes());
                        }
                        
                        Ok(response)
                    } else {
                        // Key exists but is not a set, return error
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