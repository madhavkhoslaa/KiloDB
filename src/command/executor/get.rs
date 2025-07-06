use crate::command::command_enum::Command;
use crate::store::string_store::StringStore;
use crate::store_containers::core_context::context;
use crate::traits::command::commandExecutor;
use std::error::Error;

pub struct get;

impl commandExecutor for get {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>> {
        match commandObject {
            Command::GET { key } => {
                match context.DataBase.store.get(key.as_str()) {
                    Some(Some(weak_ref)) => {
                        match weak_ref.upgrade() {
                            Some(store_ref) => {
                                let store = store_ref.borrow();
                                if let Some(string_store) =
                                    (&*store as &dyn std::any::Any).downcast_ref::<StringStore>()
                                {
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
}
