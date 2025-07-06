use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::rc::{Rc, Weak};
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use stream_resp::{RespType, RespParser};
use std::any::Any;

// Command pattern implementation
#[derive(Debug, Clone)]
enum Command {
    Ping,
    Echo(String),
    Set { key: String, value: String },
    Get { key: String },
    Quit,
    Unknown(String),
}

impl Command {
    fn from_resp_array(args: Vec<RespType>) -> Self {
        if args.is_empty() {
            return Command::Unknown("".to_string());
        }

        let command_name = match &args[0] {
            RespType::BulkString(name) => name.to_lowercase(),
            RespType::SimpleString(name) => name.to_lowercase(),
            _ => return Command::Unknown("".to_string()),
        };

        match command_name.as_str() {
            "ping" => Command::Ping,
            "echo" => {
                if args.len() >= 2 {
                    match &args[1] {
                        RespType::BulkString(msg) => Command::Echo(msg.clone()),
                        RespType::SimpleString(msg) => Command::Echo(msg.clone()),
                        _ => Command::Unknown(command_name),
                    }
                } else {
                    Command::Unknown(command_name)
                }
            }
            "set" => {
                if args.len() >= 3 {
                    let key = match &args[1] {
                        RespType::BulkString(k) => k.clone(),
                        RespType::SimpleString(k) => k.clone(),
                        _ => return Command::Unknown(command_name),
                    };
                    let value = match &args[2] {
                        RespType::BulkString(v) => v.clone(),
                        RespType::SimpleString(v) => v.clone(),
                        _ => return Command::Unknown(command_name),
                    };
                    Command::Set { key, value }
                } else {
                    Command::Unknown(command_name)
                }
            }
            "get" => {
                if args.len() >= 2 {
                    let key = match &args[1] {
                        RespType::BulkString(k) => k.clone(),
                        RespType::SimpleString(k) => k.clone(),
                        _ => return Command::Unknown(command_name),
                    };
                    Command::Get { key }
                } else {
                    Command::Unknown(command_name)
                }
            }
            "quit" => Command::Quit,
            _ => Command::Unknown(command_name),
        }
    }
}

// Command executor trait
trait CommandExecutor {
    fn execute(&mut self, command: Command) -> RespType;
}

// Implementation for your stores
impl CommandExecutor for (DictStore, TTLStore) {
    fn execute(&mut self, command: Command) -> RespType {
        let (data_store, ttl_store) = self;
        
        match command {
            Command::Ping => RespType::SimpleString("PONG".to_string()),
            
            Command::Echo(message) => RespType::BulkString(message),
            
            Command::Set { key, value } => {
                let shared_store = Rc::new(RefCell::new(StringStore::new(value)));
                data_store.insert(key.clone(), Some(Rc::downgrade(&shared_store)));
                RespType::SimpleString("OK".to_string())
            }
            
            Command::Get { key } => {
                match data_store.store.get(&key) {
                    Some(Some(weak_ref)) => {
                        if let Some(rc_ref) = weak_ref.upgrade() {
                            if let Ok(store) = rc_ref.try_borrow() {
                                if let Some(string_store) = store.as_any().downcast_ref::<StringStore>() {
                                    return RespType::BulkString(string_store.value.clone());
                                }
                            }
                        }
                        RespType::Null
                    }
                    _ => RespType::Null,
                }
            }
            
            Command::Quit => RespType::SimpleString("OK".to_string()),
            
            Command::Unknown(cmd) => RespType::Error(format!("ERR unknown command '{}'", cmd)),
        }
    }
}

trait Store: Debug {
    fn as_any(&self) -> &dyn Any;
}

trait Set {
    fn set(&mut self);
}

trait Get {
    fn get(&self);
}

#[derive(Debug)]
struct StringStore {
    value: String,
}

impl StringStore {
    fn new(value: String) -> Self {
        StringStore { value }
    }
}

impl Store for StringStore {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct DictStore {
    store: HashMap<String, Option<Weak<RefCell<dyn Store>>>>,
}

impl DictStore {
    fn new() -> Self {
        DictStore {
            store: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: Option<Weak<RefCell<dyn Store>>>) {
        self.store.insert(key, value);
    }

    fn cleanup(&mut self) {
        self.store.retain(|_k, weak_opt| match weak_opt {
            Some(weak) => weak.upgrade().is_some(),
            None => false,
        });
    }
}

#[derive(Debug)]
struct TTLStore {
    store: HashMap<usize, Rc<RefCell<dyn Store>>>,
}

impl TTLStore {
    fn new() -> Self {
        TTLStore {
            store: HashMap::new(),
        }
    }

    fn insert(&mut self, key: usize, value: Rc<RefCell<dyn Store>>) {
        self.store.insert(key, value);
    }

    fn remove(&mut self, key: &usize) {
        self.store.remove(key);
    }

    fn clear(&mut self) {
        self.store.clear();
    }
}

fn handle_client(mut stream: TcpStream, stores: &mut (DictStore, TTLStore)) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    let mut parser = RespParser::new();
    
    loop {
        // Read data from client
        let n = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        };

        // Parse RESP data
        let data = &buffer[..n];
        match parser.parse(data) {
            Ok(Some(resp_value)) => {
                println!("Parsed RESP: {:?}", resp_value);
                
                // Convert RESP to command
                let command = match resp_value {
                    RespType::Array(args) => Command::from_resp_array(args),
                    RespType::BulkString(cmd) => Command::from_resp_array(vec![RespType::BulkString(cmd)]),
                    RespType::SimpleString(cmd) => Command::from_resp_array(vec![RespType::SimpleString(cmd)]),
                    _ => Command::Unknown("Invalid command format".to_string()),
                };
                
                println!("Command: {:?}", command);
                
                // Execute command
                let response = stores.execute(command);
                
                // Send response back to client
                let response_bytes = response.to_bytes();
                stream.write_all(&response_bytes)?;
                stream.flush()?;
                
                // Check if client wants to quit
                if let RespType::SimpleString(_) = response {
                    // Could add quit logic here if needed
                }
            }
            Ok(None) => {
                // Incomplete RESP data, continue reading
                continue;
            }
            Err(e) => {
                eprintln!("Error parsing RESP: {}", e);
                let error_response = RespType::Error("ERR Invalid RESP format".to_string());
                let error_bytes = error_response.to_bytes();
                stream.write_all(&error_bytes)?;
                stream.flush()?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Redis-like server on 127.0.0.1:6379");

    let listener = TcpListener::bind("127.0.0.1:6379")?;
    println!("Server listening on 127.0.0.1:6379");

    // Handle multiple connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established from: {:?}", stream.peer_addr());
                
                // Initialize stores for this connection
                let mut data = DictStore::new();
                let mut ttl_store = TTLStore::new();

                // Set up some test data
                {
                    let shared_store: Rc<RefCell<dyn Store>> =
                        Rc::new(RefCell::new(StringStore::new("Madhav".to_owned())));
                    data.insert("Diya".to_owned(), Some(Rc::downgrade(&shared_store)));
                    ttl_store.insert(
                        SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize,
                        shared_store.clone(),
                    );
                }

                println!("Initial TTL store: {:?}", ttl_store);
                println!("Initial data store: {:?}", data);
                
                // Handle the client connection
                if let Err(e) = handle_client(stream, &mut (data, ttl_store)) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
