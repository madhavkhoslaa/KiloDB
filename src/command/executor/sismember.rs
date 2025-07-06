use crate::command::command_enum::Command;
use crate::store::set_store::SetStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::error::Error;

pub struct sismember;

impl commandExecutor for sismember {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::SISMEMBER { key, member } => {
                // Check if key exists in database
                if let Some(store) = context.DataBase.get(key) {
                    // Try to get the set store
                    if let Some(set_store) = store.borrow().downcast_ref::<SetStore>() {
                        // Key exists and is a set, check if member exists
                        let exists = set_store.contains(member);
                        Ok(format!(":{}\r\n", if exists { 1 } else { 0 }).into_bytes())
                    } else {
                        // Key exists but is not a set, return error
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