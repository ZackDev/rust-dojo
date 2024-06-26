use std::io::{BufReader, BufRead};
use std::net::TcpListener;
use std::thread;

fn main() {

    let listen_thread_handle = thread::spawn(|| {
        let tcp_listener = TcpListener::bind("127.0.0.1:9876").unwrap();
        let mut data_received: Vec<String> = Vec::new(); //macro: vec![]
        
        loop {
            println!("server entering listening loop");
            for connection in tcp_listener.incoming() {
                match connection {
                    Ok(stream) => {
                        let mut str = String::new();
                        let mut buffered_reader = BufReader::new(stream);
                        buffered_reader.read_line(&mut str).unwrap();
                        data_received.push(str);
                        println!("{:?}", data_received);
                    },
                    Err(_e) => {

                    }
                }
            }
        }
    });
    
    // intuitively, the next line isn't required, because a loop loops, right? but see:
    // - diagram without join
    //
    //      [main]      [listen]
    //      :  
    //      spawn()
    //      |           spawn()
    //      |           |
    //      exit        exit
    //                  :
    //                  loop {}
    
    listen_thread_handle.join().unwrap();
    
    // the same diagram
    // - diagram with join
    //
    //      [main]      [listen]
    //      :           
    //      spawn()
    //      |           spawn()
    //      |           |
    //      <---join()--|
    //      |           |
    //      |           loop {}
    //      |           |
    //      :           :
    //      <---break---|

    println!("server exiting");
}