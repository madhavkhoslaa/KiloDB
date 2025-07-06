use crate::command::command_enum::Command;
use crate::store::string_store::StringStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::cell::RefCell;
use std::error::Error;
use std::rc::{Rc, Weak};

pub struct set;

impl commandExecutor for set {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::SET { key, value, ttl } => match ttl {
                Some(_val) => {
                    let shared_store: Rc<RefCell<dyn Store>> =
                        Rc::new(RefCell::new(StringStore::new(value.to_owned())));

                    context
                        .DataBase
                        .store
                        .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));

                    context
                        .TTLStore
                        .store
                        .insert(_val.to_owned() as usize, shared_store);
                }
                None => {
                    let shared_store: Rc<RefCell<dyn Store>> =
                        Rc::new(RefCell::new(StringStore::new(value.to_owned())));

                    context
                        .DataBase
                        .store
                        .insert(key.to_owned(), Some(Rc::downgrade(&shared_store)));

                    context.TTLStore.store.insert(86400, shared_store);
                }
            },
            _ => {}
        }

        Ok(b"+OK\r\n".to_vec())
    }
}
