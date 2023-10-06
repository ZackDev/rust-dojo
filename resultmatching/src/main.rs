use std::{str::FromStr, process::exit};

fn main() {
    let s: String = "256".to_string();
    /* define i & u uninitialized */
    let i: i32;     // 2^32
    let u: u8;      // 2^8  [0-255]

    match FromStr::from_str(&s) {
        Ok(v) => {
            // expected to run into this
            i = v;
            println!("{i}");
        }
        Err(e) => {
            println!("{e}");
            exit(0);
        }
    }

    match FromStr::from_str(&s) {
        Ok(v) => {
            u = v;
            println!("{u}");
        }
        Err(e) => {
            // expected to run into this
            println!("{e}");
            exit(0);
        }
    }
}