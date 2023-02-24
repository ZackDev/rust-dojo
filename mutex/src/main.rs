use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    println!("thread 0 - begin:");
    let a = Arc::new(Mutex::new(0));
    let b = a.clone();
    let c = a.clone();
    
    let handle_one = thread::spawn(move || {
        println!("thread 1 - begin: {:?}", b);
        thread::sleep(Duration::from_secs(3));
        *b.lock().unwrap() = 10;
        println!("thread 1 - end: {:?}", b);
    });

    let handle_two = thread::spawn(move || {
        println!("thread 2 - begin: {:?}", c);
        *c.lock().unwrap() = 12;
        println!("thread 2 - end: {:?}", c);
    });

    handle_one.join().unwrap();

    println!("thread 0 - end: {:?} {:?}", a, a.lock().unwrap());
}
