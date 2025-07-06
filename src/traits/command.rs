use crate::command::command_enum::Command;
use crate::store_containers::core_context::context;
use std::error::Error;

pub trait commandExecutor {
    fn execute(commandObject: &Command, context: &mut context) -> Result<Vec<u8>, Box<dyn Error>>;
}
