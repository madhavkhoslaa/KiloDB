use crate::command::command_enum::Command;
use crate::store::list_store::ListStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct rpush;

impl commandExecutor for rpush {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::RPUSH { key, values } => {
                // Check if key exists in database
                if let Some(existing_store) = context.DataBase.get(key) {
                    // Try to get the existing list store
                    if let Some(list_store) = existing_store.borrow().downcast_ref::<ListStore>() {
                        // Key exists and is a list, update it
                        let mut new_list = list_store.clone();
                        for value in values {
                            new_list.push_back(value.clone());
                        }
                        
                        // Create new store with updated list
                        let new_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(new_list));
                        context.DataBase.insert(key.clone(), new_store);
                    } else {
                        // Key exists but is not a list, return error
                        return Ok(b"-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec());
                    }
                } else {
                    // Key doesn't exist, create new list
                    let mut new_list = ListStore::new();
                    for value in values {
                        new_list.push_back(value.clone());
                    }
                    
                    let new_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(new_list));
                    context.DataBase.insert(key.clone(), new_store);
                }
                
                // Return the new length of the list
                if let Some(store) = context.DataBase.get(key) {
                    if let Some(list_store) = store.borrow().downcast_ref::<ListStore>() {
                        Ok(format!(":{}\r\n", list_store.len()).into_bytes())
                    } else {
                        Ok(b":0\r\n".to_vec())
                    }
                } else {
                    Ok(b":0\r\n".to_vec())
                }
            }
            _ => {
                Ok(b"-ERR unexpected command\r\n".to_vec())
            }
        }
    }
} 