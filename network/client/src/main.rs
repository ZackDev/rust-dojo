use std::io::Write;
use std::net::{TcpStream};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let msg = String::from("Hello, World!");
    let msg_arc: Arc<String> = Arc::new(msg);
    let msg_arc2 = msg_arc.clone();
    let h1 = thread::spawn(move || {
        let mut tcp_stream = TcpStream::connect("127.0.0.1:9876").unwrap();
        tcp_stream.write(msg_arc.as_bytes()).unwrap();
    });

    let h2 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        let mut tcp_stream = TcpStream::connect("127.0.0.1:9876").unwrap();
        tcp_stream.write(msg_arc2.as_bytes()).unwrap();
    });

    h1.join().unwrap();
    h2.join().unwrap();
}