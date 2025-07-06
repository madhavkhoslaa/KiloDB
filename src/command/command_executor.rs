use crate::command::executor::{
    del, dbsize, echo, exists, expire, flushdb, get, hdel, hexists, hget, hgetall, hset, lpop, lpush, lrange, ping, rpop, rpush, sadd, set, sismember, smembers, srem, zadd, zrange, zrem
};
use crate::traits::command::commandExecutor;
use crate::{command::command_enum::Command, store_containers::core_context::context};
use std::error::Error;
pub struct command_executor {}

impl command_executor {
    pub fn execute_command(
        command: &Command,
        context: &mut context,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        match command {
            Command::SET {
                key: _key,
                value: _value,
                ttl: _ttl,
            } => set::set::execute(command, context),
            Command::GET { key: _key } => get::get::execute(command, context),
            Command::DEL { keys: _keys } => del::del::execute(command, context),
            Command::EXISTS { keys: _keys } => exists::exists::execute(command, context),
            Command::EXPIRE {
                key: _key,
                seconds: _seconds,
            } => expire::expire::execute(command, context),
            Command::FLUSHDB => flushdb::flushdb::execute(command, context),
            Command::DBSIZE => dbsize::dbsize::execute(command, context),
            Command::ECHO { message: _message } => echo::echo::execute(command, context),
            Command::PING => ping::ping::execute(command, context),
            
            // Hash commands
            Command::HSET { key: _key, fields: _fields } => hset::hset::execute(command, context),
            Command::HGET { key: _key, field: _field } => hget::hget::execute(command, context),
            Command::HGETALL { key: _key } => hgetall::hgetall::execute(command, context),
            Command::HDEL { key: _key, fields: _fields } => hdel::hdel::execute(command, context),
            Command::HEXISTS { key: _key, field: _field } => hexists::hexists::execute(command, context),
            
            // List commands
            Command::LPUSH { key: _key, values: _values } => lpush::lpush::execute(command, context),
            Command::RPUSH { key: _key, values: _values } => rpush::rpush::execute(command, context),
            Command::LPOP { key: _key } => lpop::lpop::execute(command, context),
            Command::RPOP { key: _key } => rpop::rpop::execute(command, context),
            Command::LRANGE { key: _key, start: _start, stop: _stop } => lrange::lrange::execute(command, context),
            
            // Set commands
            Command::SADD { key: _key, members: _members } => sadd::sadd::execute(command, context),
            Command::SREM { key: _key, members: _members } => srem::srem::execute(command, context),
            Command::SMEMBERS { key: _key } => smembers::smembers::execute(command, context),
            Command::SISMEMBER { key: _key, member: _member } => sismember::sismember::execute(command, context),
            
            // Sorted set commands
            Command::ZADD { key: _key, entries: _entries } => zadd::zadd::execute(command, context),
            Command::ZREM { key: _key, members: _members } => zrem::zrem::execute(command, context),
            Command::ZRANGE { key: _key, start: _start, stop: _stop } => zrange::zrange::execute(command, context),
            
            _ => Ok(b"$-1\r\n".to_vec()),
        }
    }
}
