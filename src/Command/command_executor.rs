use crate::{store_containers::core_context::context, Command::command_enum::Command};
use std::error::Error;
pub struct command_executor {}
impl command_executor {
    pub fn execute_command(
        command: &Command,
        context: &mut context,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok([1].to_vec())
    }
}
