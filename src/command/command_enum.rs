#[derive(Debug)]
pub enum Command {
    // String commands
    SET {
        key: String,
        value: String,
        ttl: Option<u64>, // TTL in seconds (from EX or PX)
    },
    GET {
        key: String,
    },
    DEL {
        keys: Vec<String>,
    },
    EXISTS {
        keys: Vec<String>,
    },
    EXPIRE {
        key: String,
        seconds: u64,
    },
    INCR {
        key: String,
    },
    DECR {
        key: String,
    },
    INCRBY {
        key: String,
        increment: i64,
    },
    DECRBY {
        key: String,
        decrement: i64,
    },
    APPEND {
        key: String,
        value: String,
    },
    STRLEN {
        key: String,
    },
    MGET {
        keys: Vec<String>,
    },
    MSET {
        pairs: Vec<(String, String)>,
    },

    // Key management commands
    KEYS {
        pattern: String,
    },
    TYPE {
        key: String,
    },
    TTL {
        key: String,
    },
    PERSIST {
        key: String,
    },
    RENAME {
        key: String,
        newkey: String,
    },

    // Hash commands
    HSET {
        key: String,
        fields: Vec<(String, String)>, // field-value pairs
    },
    HGET {
        key: String,
        field: String,
    },
    HGETALL {
        key: String,
    },
    HDEL {
        key: String,
        fields: Vec<String>,
    },
    HEXISTS {
        key: String,
        field: String,
    },
    HLEN {
        key: String,
    },
    HKEYS {
        key: String,
    },
    HVALS {
        key: String,
    },

    // List commands
    LPUSH {
        key: String,
        values: Vec<String>,
    },
    RPUSH {
        key: String,
        values: Vec<String>,
    },
    LPOP {
        key: String,
    },
    RPOP {
        key: String,
    },
    LRANGE {
        key: String,
        start: isize,
        stop: isize,
    },
    LLEN {
        key: String,
    },
    LINDEX {
        key: String,
        index: isize,
    },

    // Set commands
    SADD {
        key: String,
        members: Vec<String>,
    },
    SREM {
        key: String,
        members: Vec<String>,
    },
    SMEMBERS {
        key: String,
    },
    SISMEMBER {
        key: String,
        member: String,
    },
    SCARD {
        key: String,
    },

    // Sorted Set
    ZADD {
        key: String,
        entries: Vec<(f64, String)>, // (score, member)
    },
    ZREM {
        key: String,
        members: Vec<String>,
    },
    ZRANGE {
        key: String,
        start: isize,
        stop: isize,
    },
    ZCARD {
        key: String,
    },
    ZRANK {
        key: String,
        member: String,
    },
    ZSCORE {
        key: String,
        member: String,
    },

    // Misc
    PING,
    ECHO {
        message: String,
    },
    FLUSHDB,
    DBSIZE,

    // Unknown or unhandled
    Unknown {
        raw: Vec<String>,
    },
}
impl Command {
    pub fn new(command: &[String]) -> Command {
        if command.is_empty() {
            return Command::Unknown { raw: vec![] };
        }

        let cmd = command[0].to_uppercase();

        match cmd.as_str() {
            // --- String commands ---
            "SET" => {
                if command.len() == 3 {
                    Command::SET {
                        key: command[1].clone(),
                        value: command[2].clone(),
                        ttl: None,
                    }
                } else if command.len() == 5 && command[3].to_uppercase() == "EX" {
                    match command[4].parse::<u64>() {
                        Ok(ttl) => Command::SET {
                            key: command[1].clone(),
                            value: command[2].clone(),
                            ttl: Some(ttl),
                        },
                        Err(_) => Command::Unknown {
                            raw: command.to_vec(),
                        },
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "GET" => {
                if command.len() == 2 {
                    Command::GET {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "DEL" => {
                if command.len() >= 2 {
                    Command::DEL {
                        keys: command[1..].to_vec(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "EXISTS" => {
                if command.len() >= 2 {
                    Command::EXISTS {
                        keys: command[1..].to_vec(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "EXPIRE" => {
                if command.len() == 3 {
                    match command[2].parse::<u64>() {
                        Ok(secs) => Command::EXPIRE {
                            key: command[1].clone(),
                            seconds: secs,
                        },
                        Err(_) => Command::Unknown {
                            raw: command.to_vec(),
                        },
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "INCR" => {
                if command.len() == 2 {
                    Command::INCR {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "DECR" => {
                if command.len() == 2 {
                    Command::DECR {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "INCRBY" => {
                if command.len() == 3 {
                    match command[2].parse::<i64>() {
                        Ok(inc) => Command::INCRBY {
                            key: command[1].clone(),
                            increment: inc,
                        },
                        Err(_) => Command::Unknown {
                            raw: command.to_vec(),
                        },
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "DECRBY" => {
                if command.len() == 3 {
                    match command[2].parse::<i64>() {
                        Ok(dec) => Command::DECRBY {
                            key: command[1].clone(),
                            decrement: dec,
                        },
                        Err(_) => Command::Unknown {
                            raw: command.to_vec(),
                        },
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "APPEND" => {
                if command.len() == 3 {
                    Command::APPEND {
                        key: command[1].clone(),
                        value: command[2].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "STRLEN" => {
                if command.len() == 2 {
                    Command::STRLEN {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "MGET" => {
                if command.len() >= 2 {
                    Command::MGET {
                        keys: command[1..].to_vec(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "MSET" => {
                if command.len() >= 3 && (command.len() - 1) % 2 == 0 {
                    let mut pairs = vec![];
                    let args = &command[1..];
                    for chunk in args.chunks(2) {
                        pairs.push((chunk[0].clone(), chunk[1].clone()));
                    }
                    Command::MSET { pairs }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }

            // --- Key management commands ---
            "KEYS" => {
                if command.len() == 2 {
                    Command::KEYS {
                        pattern: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "TYPE" => {
                if command.len() == 2 {
                    Command::TYPE {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "TTL" => {
                if command.len() == 2 {
                    Command::TTL {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "PERSIST" => {
                if command.len() == 2 {
                    Command::PERSIST {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "RENAME" => {
                if command.len() == 3 {
                    Command::RENAME {
                        key: command[1].clone(),
                        newkey: command[2].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }

            // --- Hash commands ---
            "HSET" => {
                if command.len() >= 4 && (command.len() - 2) % 2 == 0 {
                    let key = command[1].clone();
                    let mut fields = vec![];
                    let pairs = &command[2..];
                    for chunk in pairs.chunks(2) {
                        fields.push((chunk[0].clone(), chunk[1].clone()));
                    }
                    Command::HSET { key, fields }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "HGET" => {
                if command.len() == 3 {
                    Command::HGET {
                        key: command[1].clone(),
                        field: command[2].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "HGETALL" => {
                if command.len() == 2 {
                    Command::HGETALL {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "HDEL" => {
                if command.len() >= 3 {
                    Command::HDEL {
                        key: command[1].clone(),
                        fields: command[2..].to_vec(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "HEXISTS" => {
                if command.len() == 3 {
                    Command::HEXISTS {
                        key: command[1].clone(),
                        field: command[2].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "HLEN" => {
                if command.len() == 2 {
                    Command::HLEN {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "HKEYS" => {
                if command.len() == 2 {
                    Command::HKEYS {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "HVALS" => {
                if command.len() == 2 {
                    Command::HVALS {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }

            // --- List commands ---
            "LPUSH" => {
                if command.len() >= 3 {
                    Command::LPUSH {
                        key: command[1].clone(),
                        values: command[2..].to_vec(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "RPUSH" => {
                if command.len() >= 3 {
                    Command::RPUSH {
                        key: command[1].clone(),
                        values: command[2..].to_vec(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "LPOP" => {
                if command.len() == 2 {
                    Command::LPOP {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "RPOP" => {
                if command.len() == 2 {
                    Command::RPOP {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "LRANGE" => {
                if command.len() == 4 {
                    let start = command[2].parse().unwrap_or(0);
                    let stop = command[3].parse().unwrap_or(0);
                    Command::LRANGE {
                        key: command[1].clone(),
                        start,
                        stop,
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "LLEN" => {
                if command.len() == 2 {
                    Command::LLEN {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "LINDEX" => {
                if command.len() == 3 {
                    match command[2].parse::<isize>() {
                        Ok(index) => Command::LINDEX {
                            key: command[1].clone(),
                            index,
                        },
                        Err(_) => Command::Unknown {
                            raw: command.to_vec(),
                        },
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }

            // --- Set commands ---
            "SADD" => {
                if command.len() >= 3 {
                    Command::SADD {
                        key: command[1].clone(),
                        members: command[2..].to_vec(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "SREM" => {
                if command.len() >= 3 {
                    Command::SREM {
                        key: command[1].clone(),
                        members: command[2..].to_vec(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "SMEMBERS" => {
                if command.len() == 2 {
                    Command::SMEMBERS {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "SISMEMBER" => {
                if command.len() == 3 {
                    Command::SISMEMBER {
                        key: command[1].clone(),
                        member: command[2].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "SCARD" => {
                if command.len() == 2 {
                    Command::SCARD {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }

            // --- Sorted set commands ---
            "ZADD" => {
                if command.len() >= 4 && (command.len() - 2) % 2 == 0 {
                    let key = command[1].clone();
                    let mut entries = vec![];
                    let pairs = &command[2..];
                    for chunk in pairs.chunks(2) {
                        if let Ok(score) = chunk[0].parse::<f64>() {
                            entries.push((score, chunk[1].clone()));
                        } else {
                            return Command::Unknown {
                                raw: command.to_vec(),
                            };
                        }
                    }
                    Command::ZADD { key, entries }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "ZREM" => {
                if command.len() >= 3 {
                    Command::ZREM {
                        key: command[1].clone(),
                        members: command[2..].to_vec(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "ZRANGE" => {
                if command.len() == 4 {
                    let start = command[2].parse().unwrap_or(0);
                    let stop = command[3].parse().unwrap_or(0);
                    Command::ZRANGE {
                        key: command[1].clone(),
                        start,
                        stop,
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "ZCARD" => {
                if command.len() == 2 {
                    Command::ZCARD {
                        key: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "ZRANK" => {
                if command.len() == 3 {
                    Command::ZRANK {
                        key: command[1].clone(),
                        member: command[2].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "ZSCORE" => {
                if command.len() == 3 {
                    Command::ZSCORE {
                        key: command[1].clone(),
                        member: command[2].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }

            // --- Misc ---
            "PING" => Command::PING,
            "ECHO" => {
                if command.len() == 2 {
                    Command::ECHO {
                        message: command[1].clone(),
                    }
                } else {
                    Command::Unknown {
                        raw: command.to_vec(),
                    }
                }
            }
            "FLUSHDB" => Command::FLUSHDB,
            "DBSIZE" => Command::DBSIZE,

            // --- Fallback ---
            _ => Command::Unknown {
                raw: command.to_vec(),
            },
        }
    }
}
