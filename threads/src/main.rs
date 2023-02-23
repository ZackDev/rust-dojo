use std::thread;
use std::time::Duration;

fn main() {
    //ex_1();
    _ex_2();
}

/**
 * 
 */
fn _ex_1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(3));
    }

    handle.join().unwrap();
}

fn _ex_2() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    //drop(v);

    handle.join().unwrap();
}
