use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use std::error::Error;

pub struct exists;

impl commandExecutor for exists {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::EXISTS { keys } => {
                let mut count = 0;
                for key in keys {
                    match context.DataBase.store.get(key.as_str()) {
                        Some(Some(weak_ref)) => {
                            // Check if the weak reference is still valid
                            if weak_ref.upgrade().is_some() {
                                count += 1;
                            }
                        }
                        Some(None) | None => {
                            // Key doesn't exist
                        }
                    }
                }
                Ok(format!(":{}\r\n", count).into_bytes())
            }
            _ => Ok(b"-ERR wrong command\r\n".to_vec()),
        }
    }
}
