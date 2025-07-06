use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use std::error::Error;

pub struct expire;

impl commandExecutor for expire {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::EXPIRE { key, seconds } => {
                // Check if the key exists in the database
                match context.DataBase.store.get(key.as_str()) {
                    Some(Some(weak_ref)) => {
                        // Try to upgrade the weak reference
                        if let Some(store_ref) = weak_ref.upgrade() {
                            // Add to TTL store with the specified expiration time
                            context.TTLStore.store.insert(*seconds as usize, store_ref);
                            Ok(b":1\r\n".to_vec()) // Return 1 to indicate success
                        } else {
                            // Weak reference is invalid, key has expired
                            Ok(b":0\r\n".to_vec()) // Return 0 to indicate key doesn't exist
                        }
                    }
                    Some(None) | None => {
                        // Key doesn't exist
                        Ok(b":0\r\n".to_vec()) // Return 0 to indicate key doesn't exist
                    }
                }
            }
            _ => Ok(b"-ERR wrong command\r\n".to_vec()),
        }
    }
}
