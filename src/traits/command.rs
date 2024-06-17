use std::net::TcpStream;

pub trait command {
    fn handle(tcp_stream: TcpStream);
}
