use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, IoSliceMut};

fn main() {
    // input as parameters
    let args: Vec<String> = std::env::args().collect();
    dbg!(args);

    // same, but doesn't panick on non-unicode parameters
    let args_os: Vec<OsString> = std::env::args_os().collect();
    dbg!(args_os);

    loop {
        // input from stdin
        match read_stdin() {
            
            Ok(path) => {
            
                // input from file
                /*
                match read_and_print(path) {
                    Ok(_) => {
                        println!("\nfile successfully read and printed.");
                        break;
                    }
                    Err(_) => {
                        println!("file not found.");
                    }
                };
                */
                
                match read_by_char_into_buckets(path) {
                    Ok(_) => todo!(),
                    Err(_) => todo!(),
                }
                
            }
            Err(_) => {}
        }
    }
}

fn read_and_print(file_path: String) -> Result<(), std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let lines: Vec<&str> = content.split("\n").collect();
    let mut lineno = 0;
    println!();
    for line in lines {
        lineno += 1;
        println!("{}:\t{}", lineno, line);
    }
    Ok(())
}

fn read_stdin() -> Result<String, std::io::Error> {
    println!("enter path to file:");
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn read_by_char_into_buckets(file_path: String) -> Result<(), std::io::Error> {
    let mut bucket: Vec<Vec<char>> = vec![vec![]];
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let chars: Vec<char> = content.chars().collect();
    let mut charno = 0;
    for c in chars {
        charno += 1;
        println!("{}:\t{}", charno, c);
    }
    println!("{:?}", content);
    Ok(())
}