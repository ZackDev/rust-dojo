use std::io::Write;
use std::net::{TcpStream};

fn main() {
    let mut tcp_stream = TcpStream::connect("127.0.0.1:9876").unwrap();
    let msg = String::from("Hello, World!");
    tcp_stream.write(msg.as_bytes()).unwrap();
}