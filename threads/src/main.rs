use std::alloc::System;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::time::SystemTime;

fn main() {
    //ex_1();
    _ex_3();
    _ex_4();
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

// how many concurrent threads are possible?
// which is faster? a) 1000 threads sqrt or b) main doing 1000x sqrt?
// a = 24ms, b = 15µs
// recap: ms = 1s/1000, µs = 1s/1000000
// creating threads seems to be very expensive!
fn _ex_3() {
    let st = SystemTime::now();
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for i in 1..1000 {
        let h = thread::spawn(move || {
            let x = (i as f32).sqrt();
        });
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{:?}", SystemTime::now().duration_since(st));
}

fn _ex_4() {
    let st = SystemTime::now();
    for i in 1..1000 {
        let y = (i as f32).sqrt();
    }
    println!("{:?}", SystemTime::now().duration_since(st));
}