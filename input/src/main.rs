use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;

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
                    Ok((u_chars, bucket)) => {
                        println!("{:?}", u_chars);
                        println!("{:?}", bucket);
                    },
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

fn read_by_char_into_buckets(file_path: String) -> Result<(Vec<char>, HashMap<char, Vec<i32>>), std::io::Error> {
    let mut bucket: HashMap<char, Vec<i32>> = HashMap::new();
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let chars: Vec<char> = content.chars().collect();
    let mut u_chars = chars.clone();
    u_chars.sort();
    u_chars.dedup();
    for u in u_chars.clone() {
        let mut indizes: Vec<i32> = vec![];
        let mut index = 0;
        for c in chars.clone() {
            if c == u {
                indizes.push(index);
            }
            index += 1;
        }
        bucket.insert(u, indizes);
    }
    Ok((u_chars, bucket))
}