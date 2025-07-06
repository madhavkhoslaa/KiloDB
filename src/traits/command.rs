use crate::Command::command_enum::Command;

pub trait commandExecutor {
    fn execute(commandObject: Command) -> Vec<u8>;
}
