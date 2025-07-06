use crate::command::command_enum::Command;
use crate::store::sorted_set_store::SortedSetStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct zrem;

impl commandExecutor for zrem {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::ZREM { key, members } => {
                // Check if key exists in database
                if let Some(store) = context.DataBase.get(key) {
                    // Try to get the sorted set store
                    if let Some(sorted_set_store) = store.borrow().downcast_ref::<SortedSetStore>() {
                        // Key exists and is a sorted set, remove members
                        let mut new_sorted_set = sorted_set_store.clone();
                        let mut removed_count = 0;
                        
                        for member in members {
                            if new_sorted_set.remove(member) {
                                removed_count += 1;
                            }
                        }
                        
                        // Update the store with the modified sorted set
                        let new_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(new_sorted_set));
                        context.DataBase.insert(key.clone(), new_store);
                        
                        Ok(format!(":{}\r\n", removed_count).into_bytes())
                    } else {
                        // Key exists but is not a sorted set, return error
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