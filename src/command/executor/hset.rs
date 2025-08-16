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
                let mut fields_added = 0;
                
                match context.DataBase.store.get(key.as_str()) {
                    Some(Some(weak_ref)) => {
                        match weak_ref.upgrade() {
                            Some(store_ref) => {
                                let mut store = store_ref.borrow_mut();
                                if let Some(hash_store) = (&mut *store as &mut dyn std::any::Any).downcast_mut::<HashStore>() {
                                    for (field, value) in fields {
                                        if hash_store.set_field(field, value) {
                                            fields_added += 1;
                                        }
                                    }
                                    Ok(format!(":{}\r\n", fields_added).into_bytes())
                                } else {
                                    Ok(b"-ERR WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                                }
                            }
                            None => {
                                // Create new hash
                                let mut new_hash = HashStore::new();
                                for (field, value) in fields {
                                    if new_hash.set_field(field, value) {
                                        fields_added += 1;
                                    }
                                }
                                let shared_store: Rc<RefCell<dyn Store>> =
                                    Rc::new(RefCell::new(new_hash));
                                context
                                    .DataBase
                                    .store
                                    .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));
                                context.TTLStore.store.insert(86400, shared_store);
                                Ok(format!(":{}\r\n", fields_added).into_bytes())
                            }
                        }
                    }
                    Some(None) | None => {
                        // Create new hash
                        let mut new_hash = HashStore::new();
                        for (field, value) in fields {
                            if new_hash.set_field(field, value) {
                                fields_added += 1;
                            }
                        }
                        let shared_store: Rc<RefCell<dyn Store>> =
                            Rc::new(RefCell::new(new_hash));
                        context
                            .DataBase
                            .store
                            .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));
                        context.TTLStore.store.insert(86400, shared_store);
                        Ok(format!(":{}\r\n", fields_added).into_bytes())
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
    fn test_hset_new_hash() {
        let mut ctx = create_test_context();
        let command = Command::HSET {
            key: "user:1".to_string(),
            fields: vec![
                ("name".to_string(), "John".to_string()),
                ("age".to_string(), "30".to_string()),
            ],
        };
        
        let result = hset::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":2\r\n");
    }

    #[test]
    fn test_hset_existing_hash() {
        let mut ctx = create_test_context();
        
        // Create existing hash
        let mut existing_hash = HashStore::new();
        existing_hash.set_field("name", "John");
        let shared_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(existing_hash));
        ctx.DataBase.store.insert("user:1".to_string(), Some(Rc::downgrade(&shared_store)));
        ctx.TTLStore.store.insert(86400, shared_store);
        
        let command = Command::HSET {
            key: "user:1".to_string(),
            fields: vec![
                ("name".to_string(), "Jane".to_string()), // Update existing
                ("age".to_string(), "25".to_string()),    // Add new
            ],
        };
        
        let result = hset::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":1\r\n"); // Only one new field added
    }

    #[test]
    fn test_hset_single_field() {
        let mut ctx = create_test_context();
        let command = Command::HSET {
            key: "user:1".to_string(),
            fields: vec![("name".to_string(), "John".to_string())],
        };
        
        let result = hset::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":1\r\n");
    }

    #[test]
    fn test_hset_empty_fields() {
        let mut ctx = create_test_context();
        let command = Command::HSET {
            key: "user:1".to_string(),
            fields: vec![],
        };
        
        let result = hset::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":0\r\n");
    }

    #[test]
    fn test_hset_wrong_command() {
        let mut ctx = create_test_context();
        let command = Command::GET { key: "test".to_string() };
        
        let result = hset::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b"-ERR wrong command\r\n");
    }
} 