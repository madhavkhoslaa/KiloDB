use std::sync::Arc;

use crate::enums::operations::OPCODES;

#[derive(Debug)]
pub struct commandParser {
    args: Vec<String>,
    op_code: OPCODES,
}

impl commandParser {
    // *4(Number of arguments)
    // $3
    // SET
    // $3
    // KeY
    // $5
    // Valu3

    pub fn new(resp_string: String) -> Result<commandParser, ()> {
        let mut dirty_args: Vec<String> = resp_string.split('\n').map(|s| s.to_string()).collect();
        let mut clean_args: Vec<String> = dirty_args
            .into_iter()
            .map(|s| s.replace("*", "").replace("\r", "").replace("$", ""))
            .collect();
        // set opcode from the clean_args array
        let opcode: String = clean_args[2].to_uppercase();
        return Ok(commandParser {
            args: clean_args,
            op_code: opcode.parse::<OPCODES>().unwrap(),
        });
    }
}
