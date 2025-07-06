mod command;
mod store;
mod store_containers;
mod traits;
use crate::command::command_executor;
use crate::command::command_enum::Command;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use store_containers::core_context::context;
// fn main() -> Result<(), Box<dyn Error>> {
//     let mut data = DictStore::new();
//     let mut ttl_store = TTLStore::new();
//     {
//         let shared_store: Rc<RefCell<dyn Store>> =
//             Rc::new(RefCell::new(StringStore::new("Madhav".to_owned())));
//         data.store
//             .insert("Diya".to_owned(), Some(Rc::downgrade(&shared_store)));
//         ttl_store.store.insert(
//             SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize,
//             shared_store.clone(),
//         );
//     }
//     println!("{:?}", ttl_store);
//     println!("{:?}", data);
//     ttl_store.store.clear();
//     println!("{:?}", ttl_store);
//     println!(
//         "{:?}",
//         data.store.get("Diya").unwrap().as_ref().unwrap().upgrade()
//     );
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     Ok(())
// }

// RESP array parser
fn stream_resp(input: &str) -> Result<Vec<String>, String> {
    let mut lines = input.split_terminator("\r\n");

    let header = lines.next().ok_or("Empty input")?;

    if !header.starts_with('*') {
        return Err("Expected RESP Array".to_string());
    }

    let num_elements: usize = header[1..]
        .parse()
        .map_err(|_| "Invalid array length".to_string())?;

    let mut result = Vec::with_capacity(num_elements);

    for _ in 0..num_elements {
        let len_line = lines.next().ok_or("Missing $length")?;
        if !len_line.starts_with('$') {
            return Err("Expected Bulk String".to_string());
        }

        let len: usize = len_line[1..]
            .parse()
            .map_err(|_| "Invalid bulk string length".to_string())?;

        let data = lines.next().ok_or("Missing data")?;

        if data.len() != len {
            return Err("Bulk string length mismatch".to_string());
        }

        result.push(data.to_string());
    }

    Ok(result)
}

// Client handler
fn handle_client(mut stream: TcpStream, context: &mut context) -> std::io::Result<()> {
    let peer = stream.peer_addr()?;
    println!("Connected to: {}", peer);
    // The context is now shared across all clients and lives for the entire program lifetime
    let mut buffer = [0u8; 512];

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            println!("Client {} disconnected.", peer);
            break;
        }

        let received = &buffer[..bytes_read];
        let input = String::from_utf8_lossy(received);

        println!("Received from {}:\n{}", peer, input);

        // --- Parse RESP ---
        let command = match stream_resp(&input) {
            Ok(cmd) => cmd,
            Err(e) => {
                let err_msg = format!("-ERR {}\r\n", e);
                stream.write_all(err_msg.as_bytes())?;
                continue;
            }
        };

        // --- Execute ---
        println!("{:?}", command);
        let mut response = b"+OK\r\n".to_vec();
        let command_object = Command::new(command.as_slice());

        match &command_object {
            Command::Unknown { .. } => {
                response = b"-ERR empty command\r\n".to_vec()
            }
            _ => {
                response =
                    command_executor::command_executor::execute_command(&command_object, context)
                        .unwrap_or(b"-ERR empty command\r\n".to_vec());
            }
        }
        println!("{:#?}", command_object);
        stream.write_all(&response)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // Create the singleton context that will live for the entire program lifetime
    let mut shared_context = context::new();
    println!("Created singleton context for the entire program lifetime");

    let listener = TcpListener::bind("127.0.0.1:6379")?;
    println!("TCP server (single-threaded) listening on Redis port 6379");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream, &mut shared_context) {
                    eprintln!("Client error: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
