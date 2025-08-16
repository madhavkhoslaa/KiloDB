use crate::command::command_enum::Command;
use crate::store::hash_store::HashStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use std::error::Error;

pub struct hget;

impl commandExecutor for hget {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::HGET { key, field } => {
                match context.DataBase.store.get(key.as_str()) {
                    Some(Some(weak_ref)) => {
                        match weak_ref.upgrade() {
                            Some(store_ref) => {
                                let store = store_ref.borrow();
                                if let Some(hash_store) = (&*store as &dyn std::any::Any).downcast_ref::<HashStore>() {
                                    match hash_store.get_field(field) {
                                        Some(value) => Ok(format!("${}\r\n{}\r\n", value.len(), value).into_bytes()),
                                        None => Ok(b"$-1\r\n".to_vec()),
                                    }
                                } else {
                                    Ok(b"-ERR WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                                }
                            }
                            None => Ok(b"$-1\r\n".to_vec()),
                        }
                    }
                    Some(None) | None => Ok(b"$-1\r\n".to_vec()),
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
    use crate::traits::Store::Store;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn create_test_context() -> context {
        context {
            DataBase: DictStore::new(),
            TTLStore: TTLStore::new(),
            EvictionStore: "TODO".to_owned(),
        }
    }

    #[test]
    fn test_hget_existing_field() {
        let mut ctx = create_test_context();
        
        // Create hash with data
        let mut hash = HashStore::new();
        hash.set_field("name", "John");
        hash.set_field("age", "30");
        let shared_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(hash));
        ctx.DataBase.store.insert("user:1".to_string(), Some(Rc::downgrade(&shared_store)));
        ctx.TTLStore.store.insert(86400, shared_store);
        
        let command = Command::HGET {
            key: "user:1".to_string(),
            field: "name".to_string(),
        };
        
        let result = hget::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b"$4\r\nJohn\r\n");
    }

    #[test]
    fn test_hget_nonexistent_field() {
        let mut ctx = create_test_context();
        
        // Create hash with data
        let mut hash = HashStore::new();
        hash.set_field("name", "John");
        let shared_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(hash));
        ctx.DataBase.store.insert("user:1".to_string(), Some(Rc::downgrade(&shared_store)));
        ctx.TTLStore.store.insert(86400, shared_store);
        
        let command = Command::HGET {
            key: "user:1".to_string(),
            field: "age".to_string(),
        };
        
        let result = hget::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b"$-1\r\n");
    }

    #[test]
    fn test_hget_nonexistent_key() {
        let mut ctx = create_test_context();
        
        let command = Command::HGET {
            key: "user:1".to_string(),
            field: "name".to_string(),
        };
        
        let result = hget::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b"$-1\r\n");
    }

    #[test]
    fn test_hget_empty_value() {
        let mut ctx = create_test_context();
        
        // Create hash with empty value
        let mut hash = HashStore::new();
        hash.set_field("empty", "");
        let shared_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(hash));
        ctx.DataBase.store.insert("user:1".to_string(), Some(Rc::downgrade(&shared_store)));
        ctx.TTLStore.store.insert(86400, shared_store);
        
        let command = Command::HGET {
            key: "user:1".to_string(),
            field: "empty".to_string(),
        };
        
        let result = hget::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b"$0\r\n\r\n");
    }

    #[test]
    fn test_hget_unicode_value() {
        let mut ctx = create_test_context();
        
        // Create hash with unicode value
        let mut hash = HashStore::new();
        hash.set_field("greeting", "Hello 世界");
        let shared_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(hash));
        ctx.DataBase.store.insert("user:1".to_string(), Some(Rc::downgrade(&shared_store)));
        ctx.TTLStore.store.insert(86400, shared_store);
        
        let command = Command::HGET {
            key: "user:1".to_string(),
            field: "greeting".to_string(),
        };
        
        let result = hget::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b"$12\r\nHello \xe4\xb8\x96\xe7\x95\x8c\r\n");
    }

    #[test]
    fn test_hget_wrong_command() {
        let mut ctx = create_test_context();
        let command = Command::GET { key: "test".to_string() };
        
        let result = hget::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b"-ERR wrong command\r\n");
    }
} 