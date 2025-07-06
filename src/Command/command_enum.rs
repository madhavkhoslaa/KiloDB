pub enum Command {
    SET {
        value: String,
        key: String,
        TTL: Option<String>,
    },
    GET {
        value: String,
    },
}

// pub enum CommandStoreMapping {
//     SET: "StringStore"
// }
