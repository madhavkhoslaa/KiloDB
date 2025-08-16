use crate::command::command_enum::Command;
use crate::store::string_store::StringStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct decr;

impl commandExecutor for decr {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::DECR { key } => {
                match context.DataBase.store.get(key.as_str()) {
                    Some(Some(weak_ref)) => {
                        match weak_ref.upgrade() {
                            Some(store_ref) => {
                                let mut store = store_ref.borrow_mut();
                                if let Some(string_store) = (&mut *store as &mut dyn std::any::Any).downcast_mut::<StringStore>() {
                                    match string_store.get_value().parse::<i64>() {
                                        Ok(current_val) => {
                                            let new_val = current_val - 1;
                                            *string_store = StringStore::new(new_val.to_string());
                                            Ok(format!(":{}\r\n", new_val).into_bytes())
                                        }
                                        Err(_) => Ok(b"-ERR value is not an integer or out of range\r\n".to_vec()),
                                    }
                                } else {
                                    Ok(b"-ERR WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                                }
                            }
                            None => {
                                // Key expired or deleted, treat as 0
                                let shared_store: Rc<RefCell<dyn Store>> =
                                    Rc::new(RefCell::new(StringStore::new("-1".to_string())));
                                context
                                    .DataBase
                                    .store
                                    .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));
                                context.TTLStore.store.insert(86400, shared_store);
                                Ok(b":-1\r\n".to_vec())
                            }
                        }
                    }
                    Some(None) | None => {
                        // Key doesn't exist, start with -1
                        let shared_store: Rc<RefCell<dyn Store>> =
                            Rc::new(RefCell::new(StringStore::new("-1".to_string())));
                        context
                            .DataBase
                            .store
                            .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));
                        context.TTLStore.store.insert(86400, shared_store);
                        Ok(b":-1\r\n".to_vec())
                    }
                }
            }
            _ => Ok(b"-ERR wrong command\r\n".to_vec()),
        }
    }
} 