use crate::command::command_enum::Command;
use crate::store::hash_store::HashStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct hset;

impl commandExecutor for hset {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::HSET { key, fields } => {
                // Check if key exists in database
                if let Some(existing_store) = context.DataBase.get(key) {
                    // Try to get the existing hash store
                    if let Some(hash_store) = existing_store.borrow().downcast_ref::<HashStore>() {
                        // Key exists and is a hash, update it
                        let mut new_hash = hash_store.clone();
                        for (field, value) in fields {
                            new_hash.set(field.clone(), value.clone());
                        }
                        
                        // Create new store with updated hash
                        let new_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(new_hash));
                        context.DataBase.insert(key.clone(), new_store);
                    } else {
                        // Key exists but is not a hash, return error
                        return Ok(b"-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec());
                    }
                } else {
                    // Key doesn't exist, create new hash
                    let mut new_hash = HashStore::new();
                    for (field, value) in fields {
                        new_hash.set(field.clone(), value.clone());
                    }
                    
                    let new_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(new_hash));
                    context.DataBase.insert(key.clone(), new_store);
                }
                
                // Return the number of fields that were added
                Ok(format!(":{}\r\n", fields.len()).into_bytes())
            }
            _ => {
                Ok(b"-ERR unexpected command\r\n".to_vec())
            }
        }
    }
} 