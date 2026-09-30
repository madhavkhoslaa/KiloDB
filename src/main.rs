mod command;
mod store;
mod store_containers;
mod traits;
use crate::command::command_enum::Command;
use crate::command::command_executor;
use crate::command::executor::{dbsize, echo, exists, get, hget, ping};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use store_containers::core_context::context;

// commands that never touch the store get dispatched under a read lock,
// so e.g. a pile of concurrent GETs can run in parallel instead of
// queueing behind each other the way they would under the write lock
fn dispatch_read_only(command_object: &Command, ctx: &context) -> Option<Vec<u8>> {
    match command_object {
        Command::GET { .. } => Some(get::get::execute(command_object, ctx).unwrap_or(b"-ERR empty command\r\n".to_vec())),
        Command::HGET { .. } => Some(hget::hget::execute(command_object, ctx).unwrap_or(b"-ERR empty command\r\n".to_vec())),
        Command::EXISTS { .. } => Some(exists::exists::execute(command_object, ctx).unwrap_or(b"-ERR empty command\r\n".to_vec())),
        Command::DBSIZE => Some(dbsize::dbsize::execute(command_object, ctx).unwrap_or(b"-ERR empty command\r\n".to_vec())),
        Command::PING => Some(ping::ping::execute(command_object, ctx).unwrap_or(b"-ERR empty command\r\n".to_vec())),
        Command::ECHO { .. } => Some(echo::echo::execute(command_object, ctx).unwrap_or(b"-ERR empty command\r\n".to_vec())),
        _ => None,
    }
}

// parses one RESP array out of buf if there's a full one there, and how
// many bytes it used, so the caller can drain just that and keep the rest
// (next pipelined command, or a partial one) around for later
fn try_parse_resp(buf: &[u8]) -> Result<Option<(Vec<String>, usize)>, String> {
    let input = match std::str::from_utf8(buf) {
        Ok(s) => s,
        Err(_) => return Err("Invalid UTF-8".to_string()),
    };

    let mut pos = 0usize;
    let mut lines = input.split_terminator("\r\n");

    let header = match lines.next() {
        Some(h) => h,
        None => return Ok(None), // need more data
    };
    if !header.starts_with('*') {
        return Err("Expected RESP Array".to_string());
    }
    pos += header.len() + 2;

    let num_elements: usize = header[1..]
        .parse()
        .map_err(|_| "Invalid array length".to_string())?;

    let mut result = Vec::with_capacity(num_elements);

    for _ in 0..num_elements {
        let len_line = match lines.next() {
            Some(l) => l,
            None => return Ok(None), // need more data
        };
        if !len_line.starts_with('$') {
            return Err("Expected Bulk String".to_string());
        }
        pos += len_line.len() + 2;

        let len: usize = len_line[1..]
            .parse()
            .map_err(|_| "Invalid bulk string length".to_string())?;

        let data = match lines.next() {
            Some(d) => d,
            None => return Ok(None), // need more data
        };
        if data.len() != len {
            return Err("Bulk string length mismatch".to_string());
        }
        pos += data.len() + 2;

        result.push(data.to_string());
    }

    Ok(Some((result, pos)))
}

// runs on its own thread per connection. context is shared via Arc<RwLock<..>>
// so a slow/idle client can't block everyone else from getting accepted,
// unlike the old single-threaded accept loop, and reads no longer queue
// behind each other the way they would under a plain Mutex
fn handle_client(mut stream: TcpStream, shared_context: Arc<RwLock<context>>) -> std::io::Result<()> {
    let peer = stream.peer_addr()?;
    println!("Connected to: {}", peer);

    let mut read_buf = [0u8; 4096];
    let mut pending: Vec<u8> = Vec::new();

    loop {
        // drain whatever's already buffered before reading the socket again -
        // handles pipelined commands and ones split across multiple reads
        loop {
            match try_parse_resp(&pending) {
                Ok(Some((command, consumed))) => {
                    pending.drain(..consumed);

                    let command_object = Command::new(command.as_slice());
                    if let Command::QUIT = &command_object {
                        stream.write_all(b"+OK\r\n")?;
                        println!("Client {} sent QUIT.", peer);
                        return Ok(());
                    }
                    let response = if let Command::Unknown { .. } = &command_object {
                        b"-ERR empty command\r\n".to_vec()
                    } else {
                        let read_guard = shared_context.read().unwrap();
                        match dispatch_read_only(&command_object, &read_guard) {
                            Some(resp) => resp,
                            None => {
                                drop(read_guard);
                                let mut write_guard = shared_context.write().unwrap();
                                command_executor::command_executor::execute_command(
                                    &command_object,
                                    &mut write_guard,
                                )
                                .unwrap_or(b"-ERR empty command\r\n".to_vec())
                            }
                        }
                    };
                    stream.write_all(&response)?;
                }
                Ok(None) => break, // command incomplete, need more bytes
                Err(e) => {
                    let err_msg = format!("-ERR {}\r\n", e);
                    stream.write_all(err_msg.as_bytes())?;
                    pending.clear();
                    break;
                }
            }
        }

        let bytes_read = stream.read(&mut read_buf)?;
        if bytes_read == 0 {
            println!("Client {} disconnected.", peer);
            break;
        }
        pending.extend_from_slice(&read_buf[..bytes_read]);
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // shared across every connection thread now, instead of one &mut
    // on the stack (that's what kept the old version single-threaded)
    let shared_context = Arc::new(RwLock::new(context::new()));
    println!("Created shared context for the entire program lifetime");

    let listener = TcpListener::bind("127.0.0.1:6379")?;
    println!("TCP server (multithreaded) listening on Redis port 6379");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let ctx = Arc::clone(&shared_context);
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream, ctx) {
                        eprintln!("Client error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
