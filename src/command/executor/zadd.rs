use crate::command::command_enum::Command;
use crate::store::sorted_set_store::SortedSetStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct zadd;

impl commandExecutor for zadd {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::ZADD { key, entries } => {
                // Check if key exists in database
                if let Some(existing_store) = context.DataBase.get(key) {
                    // Try to get the existing sorted set store
                    if let Some(sorted_set_store) = existing_store.borrow().downcast_ref::<SortedSetStore>() {
                        // Key exists and is a sorted set, update it
                        let mut new_sorted_set = sorted_set_store.clone();
                        let mut added_count = 0;
                        
                        for (score, member) in entries {
                            if new_sorted_set.add(*score, member.clone()) {
                                added_count += 1;
                            }
                        }
                        
                        // Create new store with updated sorted set
                        let new_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(new_sorted_set));
                        context.DataBase.insert(key.clone(), new_store);
                        
                        Ok(format!(":{}\r\n", added_count).into_bytes())
                    } else {
                        // Key exists but is not a sorted set, return error
                        Ok(b"-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                    }
                } else {
                    // Key doesn't exist, create new sorted set
                    let mut new_sorted_set = SortedSetStore::new();
                    let mut added_count = 0;
                    
                    for (score, member) in entries {
                        if new_sorted_set.add(*score, member.clone()) {
                            added_count += 1;
                        }
                    }
                    
                    let new_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(new_sorted_set));
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