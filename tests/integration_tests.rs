use KiloDB::command::command_enum::Command;
use KiloDB::command::command_executor::command_executor;
use KiloDB::store_containers::core_context::context;
use KiloDB::store_containers::{DictStore::DictStore, TTLStore::TTLStore};

fn create_test_context() -> context {
    context {
        DataBase: DictStore::new(),
        TTLStore: TTLStore::new(),
        EvictionStore: "TODO".to_owned(),
    }
}

#[test]
fn test_string_operations_flow() {
    let mut ctx = create_test_context();
    
    // Test SET command
    let set_command = Command::new(&["SET".to_string(), "mykey".to_string(), "hello".to_string()]);
    let result = command_executor::execute_command(&set_command, &mut ctx).unwrap();
    assert_eq!(result, b"+OK\r\n");
    
    // Test GET command
    let get_command = Command::new(&["GET".to_string(), "mykey".to_string()]);
    let result = command_executor::execute_command(&get_command, &mut ctx).unwrap();
    assert_eq!(result, b"$5\r\nhello\r\n");
    
    // Test INCR command on new key
    let incr_command = Command::new(&["INCR".to_string(), "counter".to_string()]);
    let result = command_executor::execute_command(&incr_command, &mut ctx).unwrap();
    assert_eq!(result, b":1\r\n");
    
    // Test INCR command on existing key
    let result = command_executor::execute_command(&incr_command, &mut ctx).unwrap();
    assert_eq!(result, b":2\r\n");
    
    // Test DECR command
    let decr_command = Command::new(&["DECR".to_string(), "counter".to_string()]);
    let result = command_executor::execute_command(&decr_command, &mut ctx).unwrap();
    assert_eq!(result, b":1\r\n");
}

#[test]
fn test_hash_operations_flow() {
    let mut ctx = create_test_context();
    
    // Test HSET command
    let hset_command = Command::new(&[
        "HSET".to_string(),
        "user:1".to_string(),
        "name".to_string(),
        "John".to_string(),
        "age".to_string(),
        "30".to_string(),
    ]);
    let result = command_executor::execute_command(&hset_command, &mut ctx).unwrap();
    assert_eq!(result, b":2\r\n");
    
    // Test HGET command
    let hget_command = Command::new(&[
        "HGET".to_string(),
        "user:1".to_string(),
        "name".to_string(),
    ]);
    let result = command_executor::execute_command(&hget_command, &mut ctx).unwrap();
    assert_eq!(result, b"$4\r\nJohn\r\n");
    
    // Test HGET on non-existent field
    let hget_command = Command::new(&[
        "HGET".to_string(),
        "user:1".to_string(),
        "email".to_string(),
    ]);
    let result = command_executor::execute_command(&hget_command, &mut ctx).unwrap();
    assert_eq!(result, b"$-1\r\n");
}

#[test]
fn test_list_operations_flow() {
    let mut ctx = create_test_context();
    
    // Test LPUSH command
    let lpush_command = Command::new(&[
        "LPUSH".to_string(),
        "mylist".to_string(),
        "item1".to_string(),
        "item2".to_string(),
        "item3".to_string(),
    ]);
    let result = command_executor::execute_command(&lpush_command, &mut ctx).unwrap();
    assert_eq!(result, b":3\r\n");
    
    // Test another LPUSH on existing list
    let lpush_command = Command::new(&[
        "LPUSH".to_string(),
        "mylist".to_string(),
        "item0".to_string(),
    ]);
    let result = command_executor::execute_command(&lpush_command, &mut ctx).unwrap();
    assert_eq!(result, b":4\r\n");
}

#[test]
fn test_set_operations_flow() {
    let mut ctx = create_test_context();
    
    // Test SADD command
    let sadd_command = Command::new(&[
        "SADD".to_string(),
        "myset".to_string(),
        "member1".to_string(),
        "member2".to_string(),
        "member3".to_string(),
    ]);
    let result = command_executor::execute_command(&sadd_command, &mut ctx).unwrap();
    assert_eq!(result, b":3\r\n");
    
    // Test SADD with duplicate members
    let sadd_command = Command::new(&[
        "SADD".to_string(),
        "myset".to_string(),
        "member1".to_string(), // Duplicate
        "member4".to_string(), // New
    ]);
    let result = command_executor::execute_command(&sadd_command, &mut ctx).unwrap();
    assert_eq!(result, b":1\r\n"); // Only 1 new member added
}

#[test]
fn test_sorted_set_operations_flow() {
    let mut ctx = create_test_context();
    
    // Test ZADD command
    let zadd_command = Command::new(&[
        "ZADD".to_string(),
        "myzset".to_string(),
        "1.0".to_string(),
        "first".to_string(),
        "2.0".to_string(),
        "second".to_string(),
        "3.0".to_string(),
        "third".to_string(),
    ]);
    let result = command_executor::execute_command(&zadd_command, &mut ctx).unwrap();
    assert_eq!(result, b":3\r\n");
    
    // Test ZADD with score update
    let zadd_command = Command::new(&[
        "ZADD".to_string(),
        "myzset".to_string(),
        "1.5".to_string(),
        "first".to_string(), // Update existing member
        "4.0".to_string(),
        "fourth".to_string(), // Add new member
    ]);
    let result = command_executor::execute_command(&zadd_command, &mut ctx).unwrap();
    assert_eq!(result, b":1\r\n"); // Only 1 new member added
}

#[test]
fn test_database_operations_flow() {
    let mut ctx = create_test_context();
    
    // Add some data
    let set_command = Command::new(&["SET".to_string(), "key1".to_string(), "value1".to_string()]);
    command_executor::execute_command(&set_command, &mut ctx).unwrap();
    
    let set_command = Command::new(&["SET".to_string(), "key2".to_string(), "value2".to_string()]);
    command_executor::execute_command(&set_command, &mut ctx).unwrap();
    
    // Test DBSIZE
    let dbsize_command = Command::new(&["DBSIZE".to_string()]);
    let result = command_executor::execute_command(&dbsize_command, &mut ctx).unwrap();
    assert_eq!(result, b":2\r\n");
    
    // Test PING
    let ping_command = Command::new(&["PING".to_string()]);
    let result = command_executor::execute_command(&ping_command, &mut ctx).unwrap();
    assert_eq!(result, b"+PONG\r\n");
    
    // Test ECHO
    let echo_command = Command::new(&["ECHO".to_string(), "hello world".to_string()]);
    let result = command_executor::execute_command(&echo_command, &mut ctx).unwrap();
    assert_eq!(result, b"$11\r\nhello world\r\n");
}

#[test]
fn test_error_handling() {
    let mut ctx = create_test_context();
    
    // Test GET on non-existent key
    let get_command = Command::new(&["GET".to_string(), "nonexistent".to_string()]);
    let result = command_executor::execute_command(&get_command, &mut ctx).unwrap();
    assert_eq!(result, b"$-1\r\n");
    
    // Test INCR on non-numeric value
    let set_command = Command::new(&["SET".to_string(), "text".to_string(), "hello".to_string()]);
    command_executor::execute_command(&set_command, &mut ctx).unwrap();
    
    let incr_command = Command::new(&["INCR".to_string(), "text".to_string()]);
    let result = command_executor::execute_command(&incr_command, &mut ctx).unwrap();
    assert_eq!(result, b"-ERR value is not an integer or out of range\r\n");
    
    // Test unknown command
    let unknown_command = Command::new(&["UNKNOWN".to_string()]);
    let result = command_executor::execute_command(&unknown_command, &mut ctx).unwrap();
    assert_eq!(result, b"$-1\r\n");
}

#[test]
fn test_command_parsing() {
    // Test valid commands
    let set_command = Command::new(&["SET".to_string(), "key".to_string(), "value".to_string()]);
    match set_command {
        Command::SET { key, value, ttl } => {
            assert_eq!(key, "key");
            assert_eq!(value, "value");
            assert_eq!(ttl, None);
        }
        _ => panic!("Expected SET command"),
    }
    
    // Test SET with TTL
    let set_command = Command::new(&[
        "SET".to_string(),
        "key".to_string(),
        "value".to_string(),
        "EX".to_string(),
        "60".to_string(),
    ]);
    match set_command {
        Command::SET { key, value, ttl } => {
            assert_eq!(key, "key");
            assert_eq!(value, "value");
            assert_eq!(ttl, Some(60));
        }
        _ => panic!("Expected SET command with TTL"),
    }
    
    // Test invalid command
    let invalid_command = Command::new(&["INVALID".to_string(), "arg".to_string()]);
    match invalid_command {
        Command::Unknown { raw: _ } => {
            // Expected
        }
        _ => panic!("Expected Unknown command"),
    }
} 