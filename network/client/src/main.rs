use std::io::Write;
use std::net::{TcpStream};
use std::thread;

fn main() {
    let h1 = thread::spawn(|| {
        let mut tcp_stream = TcpStream::connect("127.0.0.1:9876").unwrap();
        let msg = String::from("Hello, World!");
        tcp_stream.write(msg.as_bytes()).unwrap();
    });

    let h2 = thread::spawn(|| {
        let mut tcp_stream = TcpStream::connect("127.0.0.1:9876").unwrap();
        let msg = String::from("Hello, World!");
        tcp_stream.write(msg.as_bytes()).unwrap();
    });

    h1.join().unwrap();
    h2.join().unwrap();
}