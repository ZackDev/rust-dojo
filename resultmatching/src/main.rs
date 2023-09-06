use std::{str::FromStr, process::exit};

fn main() {
    let s: String = "12".to_string();

    let i: i32;
    match FromStr::from_str(&s) {
        Ok(v) => {
            i = v;
        }
        Err(e) => {
            println!("{e}");
            exit(0);
        }
    }
    println!("{i}");
}