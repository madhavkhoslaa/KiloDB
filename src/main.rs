use std::{
    io::{self, Read, Write},
    net::TcpListener,
    thread,
};

use KiloDB::parser::command_parser::commandParser;
#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("Server listening on port 6379");

    loop {
        let (mut stream, _) = listener.accept().unwrap();
        println!("Accepted new connection");

        thread::spawn(move || {
            let mut buffer = [0; 1024];

            loop {
                match stream.read(&mut buffer) {
                    Ok(0) => break, // Connection closed
                    Ok(n) => {
                        println!("Received {} bytes", n);
                        println!("Received data: {}", String::from_utf8_lossy(&buffer[..n]));
                        let parsed_args = commandParser::parse_args_array(
                            String::from_utf8_lossy(&buffer[..n]).to_string(),
                        );
                        println!("{:?}", parsed_args);
                        if let Err(e) = stream.write_all(b"+pong\r\n") {
                            eprintln!("Failed to write to socket: {}", e);
                            break;
                        }
                        // Check if received data is "ping"
                    }
                    Err(_) => {
                        // Handle read error (e.g., connection reset by peer)
                        break;
                    }
                }

                buffer = [0; 1024]; // Clear buffer for next read
            }
        });
    }
}
