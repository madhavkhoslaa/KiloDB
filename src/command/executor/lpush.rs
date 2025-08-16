use crate::command::command_enum::Command;
use crate::store::vector_store::VectorStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct lpush;

impl commandExecutor for lpush {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::LPUSH { key, values } => {
                match context.DataBase.store.get(key.as_str()) {
                    Some(Some(weak_ref)) => {
                        match weak_ref.upgrade() {
                            Some(store_ref) => {
                                let mut store = store_ref.borrow_mut();
                                if let Some(vector_store) = (&mut *store as &mut dyn std::any::Any).downcast_mut::<VectorStore>() {
                                    for value in values.iter().rev() { // Reverse to maintain order
                                        vector_store.push_left(value);
                                    }
                                    let length = vector_store.len();
                                    Ok(format!(":{}\r\n", length).into_bytes())
                                } else {
                                    Ok(b"-ERR WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                                }
                            }
                            None => {
                                // Create new list
                                let mut new_list = VectorStore::new();
                                for value in values.iter().rev() {
                                    new_list.push_left(value);
                                }
                                let length = new_list.len();
                                let shared_store: Rc<RefCell<dyn Store>> =
                                    Rc::new(RefCell::new(new_list));
                                context
                                    .DataBase
                                    .store
                                    .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));
                                context.TTLStore.store.insert(86400, shared_store);
                                Ok(format!(":{}\r\n", length).into_bytes())
                            }
                        }
                    }
                    Some(None) | None => {
                        // Create new list
                        let mut new_list = VectorStore::new();
                        for value in values.iter().rev() {
                            new_list.push_left(value);
                        }
                        let length = new_list.len();
                        let shared_store: Rc<RefCell<dyn Store>> =
                            Rc::new(RefCell::new(new_list));
                        context
                            .DataBase
                            .store
                            .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));
                        context.TTLStore.store.insert(86400, shared_store);
                        Ok(format!(":{}\r\n", length).into_bytes())
                    }
                }
            }
            _ => Ok(b"-ERR wrong command\r\n".to_vec()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store_containers::{DictStore::DictStore, TTLStore::TTLStore};

    fn create_test_context() -> context {
        context {
            DataBase: DictStore::new(),
            TTLStore: TTLStore::new(),
            EvictionStore: "TODO".to_owned(),
        }
    }

    #[test]
    fn test_lpush_new_list() {
        let mut ctx = create_test_context();
        let command = Command::LPUSH {
            key: "mylist".to_string(),
            values: vec!["item1".to_string(), "item2".to_string(), "item3".to_string()],
        };
        
        let result = lpush::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":3\r\n");
    }

    #[test]
    fn test_lpush_existing_list() {
        let mut ctx = create_test_context();
        
        // Create existing list
        let mut existing_list = VectorStore::new();
        existing_list.push_left("existing");
        let shared_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(existing_list));
        ctx.DataBase.store.insert("mylist".to_string(), Some(Rc::downgrade(&shared_store)));
        ctx.TTLStore.store.insert(86400, shared_store);
        
        let command = Command::LPUSH {
            key: "mylist".to_string(),
            values: vec!["item1".to_string(), "item2".to_string()],
        };
        
        let result = lpush::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":3\r\n"); // 1 existing + 2 new
    }

    #[test]
    fn test_lpush_single_value() {
        let mut ctx = create_test_context();
        let command = Command::LPUSH {
            key: "mylist".to_string(),
            values: vec!["single".to_string()],
        };
        
        let result = lpush::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":1\r\n");
    }

    #[test]
    fn test_lpush_empty_values() {
        let mut ctx = create_test_context();
        let command = Command::LPUSH {
            key: "mylist".to_string(),
            values: vec![],
        };
        
        let result = lpush::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":0\r\n");
    }

    #[test]
    fn test_lpush_wrong_command() {
        let mut ctx = create_test_context();
        let command = Command::GET { key: "test".to_string() };
        
        let result = lpush::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b"-ERR wrong command\r\n");
    }
} 