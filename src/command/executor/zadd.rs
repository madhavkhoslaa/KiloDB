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
                let mut added_count = 0;
                
                match context.DataBase.store.get(key.as_str()) {
                    Some(Some(weak_ref)) => {
                        match weak_ref.upgrade() {
                            Some(store_ref) => {
                                let mut store = store_ref.borrow_mut();
                                if let Some(zset_store) = (&mut *store as &mut dyn std::any::Any).downcast_mut::<SortedSetStore>() {
                                    for (score, member) in entries {
                                        if zset_store.add_member(member, *score) {
                                            added_count += 1;
                                        }
                                    }
                                    Ok(format!(":{}\r\n", added_count).into_bytes())
                                } else {
                                    Ok(b"-ERR WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_vec())
                                }
                            }
                            None => {
                                // Create new sorted set
                                let mut new_zset = SortedSetStore::new();
                                for (score, member) in entries {
                                    if new_zset.add_member(member, *score) {
                                        added_count += 1;
                                    }
                                }
                                let shared_store: Rc<RefCell<dyn Store>> =
                                    Rc::new(RefCell::new(new_zset));
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
                        // Create new sorted set
                        let mut new_zset = SortedSetStore::new();
                        for (score, member) in entries {
                            if new_zset.add_member(member, *score) {
                                added_count += 1;
                            }
                        }
                        let shared_store: Rc<RefCell<dyn Store>> =
                            Rc::new(RefCell::new(new_zset));
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
    fn test_zadd_new_sorted_set() {
        let mut ctx = create_test_context();
        let command = Command::ZADD {
            key: "myzset".to_string(),
            entries: vec![
                (1.0, "first".to_string()),
                (2.0, "second".to_string()),
                (3.0, "third".to_string()),
            ],
        };
        
        let result = zadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":3\r\n");
    }

    #[test]
    fn test_zadd_existing_sorted_set() {
        let mut ctx = create_test_context();
        
        // Create existing sorted set
        let mut existing_zset = SortedSetStore::new();
        existing_zset.add_member("existing", 1.0);
        let shared_store: Rc<RefCell<dyn Store>> = Rc::new(RefCell::new(existing_zset));
        ctx.DataBase.store.insert("myzset".to_string(), Some(Rc::downgrade(&shared_store)));
        ctx.TTLStore.store.insert(86400, shared_store);
        
        let command = Command::ZADD {
            key: "myzset".to_string(),
            entries: vec![
                (1.5, "existing".to_string()), // Update existing member
                (2.0, "new".to_string()),      // Add new member
            ],
        };
        
        let result = zadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":1\r\n"); // Only 1 new member added
    }

    #[test]
    fn test_zadd_duplicate_scores() {
        let mut ctx = create_test_context();
        let command = Command::ZADD {
            key: "myzset".to_string(),
            entries: vec![
                (1.0, "member1".to_string()),
                (1.0, "member2".to_string()),
                (1.0, "member3".to_string()),
            ],
        };
        
        let result = zadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":3\r\n"); // All members are unique even with same score
    }

    #[test]
    fn test_zadd_negative_scores() {
        let mut ctx = create_test_context();
        let command = Command::ZADD {
            key: "myzset".to_string(),
            entries: vec![
                (-1.0, "negative".to_string()),
                (0.0, "zero".to_string()),
                (1.0, "positive".to_string()),
            ],
        };
        
        let result = zadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":3\r\n");
    }

    #[test]
    fn test_zadd_empty_entries() {
        let mut ctx = create_test_context();
        let command = Command::ZADD {
            key: "myzset".to_string(),
            entries: vec![],
        };
        
        let result = zadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":0\r\n");
    }

    #[test]
    fn test_zadd_single_entry() {
        let mut ctx = create_test_context();
        let command = Command::ZADD {
            key: "myzset".to_string(),
            entries: vec![(1.0, "single".to_string())],
        };
        
        let result = zadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b":1\r\n");
    }

    #[test]
    fn test_zadd_wrong_command() {
        let mut ctx = create_test_context();
        let command = Command::GET { key: "test".to_string() };
        
        let result = zadd::execute(&command, &mut ctx).unwrap();
        assert_eq!(result, b"-ERR wrong command\r\n");
    }
} 