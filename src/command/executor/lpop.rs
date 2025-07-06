use crate::command::command_enum::Command;
use crate::store::list_store::ListStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct lpop;

impl commandExecutor for lpop {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::LPOP { key } => {
                // Check if key exists in database
                if let Some(store) = context.DataBase.get(key) {
                    // Try to get the list store
                    if let Some(list_store) = store.borrow().downcast_ref::<ListStore>() {
                        // Key exists and is a list, pop from front
                        let mut new_list = list_store.clone();
                        if let Some(value) = new_list.pop_front() {
                            // Update the store with the modified list
                            let new_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(new_list));
                            context.DataBase.insert(key.clone(), new_store);
                            
                            // Return the popped value
                            let response = format!("${}\r\n{}\r\n", value.len(), value);
                            Ok(response.into_bytes())
                        } else {
                            // List is empty, return null
                            Ok(b"$-1\r\n".to_vec())
                        }
                    } else {
                        // Key exists but is not a list, return error
                        Ok(b"-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                    }
                } else {
                    // Key doesn't exist, return null
                    Ok(b"$-1\r\n".to_vec())
                }
            }
            _ => {
                Ok(b"-ERR unexpected command\r\n".to_vec())
            }
        }
    }
} 