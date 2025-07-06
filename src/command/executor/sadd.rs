use crate::command::command_enum::Command;
use crate::store::set_store::SetStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct sadd;

impl commandExecutor for sadd {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::SADD { key, members } => {
                // Check if key exists in database
                if let Some(existing_store) = context.DataBase.get(key) {
                    // Try to get the existing set store
                    if let Some(set_store) = existing_store.borrow().downcast_ref::<SetStore>() {
                        // Key exists and is a set, update it
                        let mut new_set = set_store.clone();
                        let mut added_count = 0;
                        
                        for member in members {
                            if new_set.add(member.clone()) {
                                added_count += 1;
                            }
                        }
                        
                        // Create new store with updated set
                        let new_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(new_set));
                        context.DataBase.insert(key.clone(), new_store);
                        
                        Ok(format!(":{}\r\n", added_count).into_bytes())
                    } else {
                        // Key exists but is not a set, return error
                        Ok(b"-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                    }
                } else {
                    // Key doesn't exist, create new set
                    let mut new_set = SetStore::new();
                    let mut added_count = 0;
                    
                    for member in members {
                        if new_set.add(member.clone()) {
                            added_count += 1;
                        }
                    }
                    
                    let new_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(new_set));
                    context.DataBase.insert(key.clone(), new_store);
                    
                    Ok(format!(":{}\r\n", added_count).into_bytes())
                }
            }
            _ => {
                Ok(b"-ERR unexpected command\r\n".to_vec())
            }
        }
    }
} 