use crate::command::command_enum::Command;
use crate::store::string_store::StringStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use crate::traits::Store::Store;
use std::sync::Mutex;
use std::error::Error;
use std::sync::Arc;

pub struct set;

impl commandExecutor for set {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::SET { key, value, ttl } => match ttl {
                Some(_val) => {
                    let shared_store: Arc<Mutex<dyn Store>> =
                        Arc::new(Mutex::new(StringStore::new(value.to_owned())));

                    context
                        .DataBase
                        .store
                        .insert(key.to_owned(), Some(Arc::downgrade(&shared_store)));

                    context.TTLStore.store.insert(key.to_owned(), shared_store);
                }
                None => {
                    let shared_store: Arc<Mutex<dyn Store>> =
                        Arc::new(Mutex::new(StringStore::new(value.to_owned())));

                    context
                        .DataBase
                        .store
                        .insert(key.to_owned(), Some(Arc::downgrade(&shared_store)));

                    context.TTLStore.store.insert(key.to_owned(), shared_store);
                }
            },
            _ => {}
        }

        Ok(b"+OK\r\n".to_vec())
    }
}
