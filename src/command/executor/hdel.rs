use crate::command::command_enum::Command;
use crate::store::hash_store::HashStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct hdel;

impl commandExecutor for hdel {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::HDEL { key, fields } => {
                // Check if key exists in database
                if let Some(store) = context.DataBase.get(key) {
                    // Try to get the hash store
                    if let Some(hash_store) = store.borrow().downcast_ref::<HashStore>() {
                        // Key exists and is a hash, delete fields
                        let mut new_hash = hash_store.clone();
                        let mut deleted_count = 0;
                        
                        for field in fields {
                            if new_hash.delete(field) {
                                deleted_count += 1;
                            }
                        }
                        
                        // Update the store with the modified hash
                        let new_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(new_hash));
                        context.DataBase.insert(key.clone(), new_store);
                        
                        // Return the number of fields deleted
                        Ok(format!(":{}\r\n", deleted_count).into_bytes())
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