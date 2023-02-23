use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let txc = tx.clone(); //cloe, because t1 takes ownership

    let t1_handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        let val = String::from("ho");
        tx.send(val).unwrap();
    });

    let t2_handle = thread::spawn(move || {
        let mut rcv_msg = 0;
        loop {
            if rcv_msg >= 3 {
                break;
            }
            let val = rx.recv();
            match val {
                Ok(s) => println!("{}", s),
                Err(e) => println!("{}", e)
            }
            rcv_msg += 1;
        }
    });

    t1_handle.join().unwrap();

    txc.send(String::from("hey")).unwrap();

    t2_handle.join().unwrap();
}
