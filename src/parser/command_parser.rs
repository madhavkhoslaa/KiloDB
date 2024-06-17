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
    pub fn parse_args_array(resp_string: String) -> Result<Vec<String>, ()> {
        let mut dirty_args: Vec<String> = resp_string.split('\n').map(|s| s.to_string()).collect();
        let clean_args: Vec<String> = dirty_args
            .into_iter()
            .map(|s| s.replace("*", "").replace("\r", "").replace("$", ""))
            .collect();
        // set opcode from the clean_args array
        return Ok(clean_args);
    }
}
