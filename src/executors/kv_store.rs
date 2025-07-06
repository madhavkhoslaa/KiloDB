use crate::store::string_store::StringStore;
use crate::traits::Store::Store;
use crate::{store_containers::core_context::context, Command::command_enum::Command};
use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
// for getting and setting simple key and values
pub struct kv_store {}

impl kv_store {
    pub fn set(command: &Command, ctx: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        println!("=== SET OPERATION ===");
        println!("Database store: {:?}", ctx.DataBase.store);
        println!("TTL store: {:?}", ctx.TTLStore.store);
        println!("=====================");
        
        match command {
            Command::SET { key, value, ttl } => match (ttl) {
                Some(_val) => {
                    let shared_store: Rc<RefCell<dyn Store>> =
                        Rc::new(RefCell::new(StringStore::new(value.to_owned())));
                    ctx.DataBase
                        .store
                        .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));
                    ctx.TTLStore
                        .store
                        .insert(ttl.unwrap() as usize, shared_store);
                }
                None => {
                    let shared_store: Rc<RefCell<dyn Store>> =
                        Rc::new(RefCell::new(StringStore::new(value.to_owned())));
                    ctx.DataBase
                        .store
                        .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));
                    ctx.TTLStore.store.insert(86400, shared_store);
                }
            },
            _ => {}
        }
        Ok(b"+OK\r\n".to_vec())
    }
    pub fn get(command: &Command, ctx: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        println!("=== GET OPERATION ===");
        println!("Database store: {:?}", ctx.DataBase.store);
        println!("TTL store: {:?}", ctx.TTLStore.store);
        println!("=====================");
        
        match command {
            Command::GET { key } => {
                match ctx.DataBase.store.get(key.as_str()) {
                    Some(Some(weak_ref)) => {
                        match weak_ref.upgrade() {
                            Some(store_ref) => {
                                let store = store_ref.borrow();
                                if let Some(string_store) = (&*store as &dyn std::any::Any).downcast_ref::<StringStore>() {
                                    let value = string_store.get_value();
                                    Ok(format!("${}\r\n{}\r\n", value.len(), value).into_bytes())
                                } else {
                                    Ok(b"-ERR value is not a string\r\n".to_vec())
                                }
                            }
                            None => Ok(b"$-1\r\n".to_vec()), // Key expired or deleted
                        }
                    }
                    Some(None) | None => Ok(b"$-1\r\n".to_vec()), // Key not found
                }
            }
            _ => Ok(b"-ERR wrong command\r\n".to_vec()),
        }
    }
    pub fn del(command: &Command, ctx: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match command {
            Command::DEL { keys } => {
                let mut deleted_count = 0;
                for key in keys {
                    if let Some(Some(weak_ref)) = ctx.DataBase.store.get(key.as_str()) {
                        if let Some(store_ref) = weak_ref.upgrade() {
                            // Remove from TTL store if it exists there
                            // This is a simplified approach - in a real implementation,
                            // you'd want to track TTL entries more efficiently
                            ctx.TTLStore
                                .store
                                .retain(|_, store| !Rc::ptr_eq(store, &store_ref));
                        }
                        ctx.DataBase.store.remove(key.as_str());
                        deleted_count += 1;
                    }
                }
                Ok(format!(":{}\r\n", deleted_count).into_bytes())
            }
            _ => Ok(b"-ERR wrong command\r\n".to_vec()),
        }
    }
    // pub fn exists(command: &Command, ctx: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {}
    // pub fn expire(command: &Command, ctx: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {}
}
