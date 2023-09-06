use std::{str::FromStr, process::exit};

fn main() {
    let s: String = "12".to_string();
    /* define i uninitialized */
    let i: i32;

    /* assign a value to i inside the Ok arm */
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