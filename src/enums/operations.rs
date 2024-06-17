use std::str::FromStr;

#[derive(Debug)]
pub enum OPCODES {
    SET,
    GET,
    DEL,
    PING,
}

impl FromStr for OPCODES {
    type Err = MyEnumParseError;
    fn from_str(op_string: &str) -> Result<Self, Self::Err> {
        let resp = match op_string.to_uppercase().as_str() {
            "SET" => OPCODES::SET,
            "GET" => OPCODES::GET,
            "DEL" => OPCODES::DEL,
            "PING" => OPCODES::PING,
            _ => OPCODES::PING,
        };
        return Ok(resp);
    }
}
#[derive(Debug)]
pub enum MyEnumParseError {
    UnknownVariant,
}
