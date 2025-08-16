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
                let mut added_count = 0;
                
                match context.DataBase.store.get(key.as_str()) {
                    Some(Some(weak_ref)) => {
                        match weak_ref.upgrade() {
                            Some(store_ref) => {
                                let mut store = store_ref.borrow_mut();
                                if let Some(set_store) = (&mut *store as &mut dyn std::any::Any).downcast_mut::<SetStore>() {
                                    for member in members {
                                        if set_store.add_member(member) {
                                            added_count += 1;
                                        }
                                    }
                                    Ok(format!(":{}\r\n", added_count).into_bytes())
                                } else {
                                    Ok(b"-ERR WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                                }
                            }
                            None => {
                                // Create new set
                                let mut new_set = SetStore::new();
                                for member in members {
                                    if new_set.add_member(member) {
                                        added_count += 1;
                                    }
                                }
                                let shared_store: Rc<RefCell<dyn Store>> =
                                    Rc::new(RefCell::new(new_set));
                                context
                                    .DataBase
                                    .store
                                    .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));
                                context.TTLStore.store.insert(86400, shared_store);
                                Ok(format!(":{}\r\n", added_count).into_bytes())
                            }
                        }
                    }
                    Some(None) | None => {
                        // Create new set
                        let mut new_set = SetStore::new();
                        for member in members {
                            if new_set.add_member(member) {
                                added_count += 1;
                            }
                        }
                        let shared_store: Rc<RefCell<dyn Store>> =
                            Rc::new(RefCell::new(new_set));
                        context
                            .DataBase
                            .store
                            .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));
                        context.TTLStore.store.insert(86400, shared_store);
                        Ok(format!(":{}\r\n", added_count).into_bytes())
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
    fn test_sadd_new_set() {
        let mut ctx = create_test_context();
        let command = Command::SADD {
            key: "myset".to_string(),
            members: vec!["member1".to_string(), "member2".to_string(), "member3".to_string()],
        };
        
        let result = sadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":3\r\n");
    }

    #[test]
    fn test_sadd_existing_set() {
        let mut ctx = create_test_context();
        
        // Create existing set
        let mut existing_set = SetStore::new();
        existing_set.add_member("existing1");
        existing_set.add_member("existing2");
        let shared_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(existing_set));
        ctx.DataBase.store.insert("myset".to_string(), Some(Rc::downgrade(&shared_store)));
        ctx.TTLStore.store.insert(86400, shared_store);
        
        let command = Command::SADD {
            key: "myset".to_string(),
            members: vec!["existing1".to_string(), "new1".to_string(), "new2".to_string()],
        };
        
        let result = sadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":2\r\n"); // Only 2 new members added
    }

    #[test]
    fn test_sadd_duplicate_members() {
        let mut ctx = create_test_context();
        let command = Command::SADD {
            key: "myset".to_string(),
            members: vec!["member1".to_string(), "member1".to_string(), "member2".to_string()],
        };
        
        let result = sadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":2\r\n"); // Only 2 unique members added
    }

    #[test]
    fn test_sadd_empty_members() {
        let mut ctx = create_test_context();
        let command = Command::SADD {
            key: "myset".to_string(),
            members: vec![],
        };
        
        let result = sadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":0\r\n");
    }

    #[test]
    fn test_sadd_single_member() {
        let mut ctx = create_test_context();
        let command = Command::SADD {
            key: "myset".to_string(),
            members: vec!["single".to_string()],
        };
        
        let result = sadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":1\r\n");
    }

    #[test]
    fn test_sadd_wrong_command() {
        let mut ctx = create_test_context();
        let command = Command::GET { key: "test".to_string() };
        
        let result = sadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b"-ERR wrong command\r\n");
    }
} 