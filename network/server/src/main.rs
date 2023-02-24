use std::io::{Read, BufReader, BufRead};
use std::net::{TcpListener};
use std::thread;

fn main() {

    let listen_thread_handle = thread::spawn(|| {
        let tcp_listener = TcpListener::bind("127.0.0.1:9876").unwrap();
        let mut data_received: Vec<String> = Vec::new();
        loop {
            println!("server entering listening loop");
            for stream in tcp_listener.incoming() {
                match stream {
                    Ok(s) => {
                        let mut str = String::new();
                        let mut buffered_reader = BufReader::new(s);
                        buffered_reader.read_line(&mut str);
                        data_received.push(str);
                        println!("{:?}", data_received);
                    },
                    Err(e) => {

                    }
                }
            }
        }
    });
    
    // intuitively, the next line isn't required. but see:
    // - diagram without join
    //      main        listen
    //
    //      spawn
    //      |           spawn
    //      |           |
    //      exit        exit
    //                  |
    //                  loop
    
    listen_thread_handle.join().unwrap();
    
    //      main        listen
    //
    //      spawn
    //      |           spawn
    //      |           |
    //      <---join----|
    //      |           |
    //      |           loop
    //      |           |
    //      |           |

    println!("server exiting");
}