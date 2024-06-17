#[derive(Debug)]
pub enum OPCODES {
    SET,
    GET,
    DEL,
    PING,
}

impl OPCODES {
    fn get_opcode(op_string: String) -> OPCODES {
        // Return OPCODE emum from a string
        todo!();
    }
}
