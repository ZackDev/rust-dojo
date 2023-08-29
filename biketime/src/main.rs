use clap::Parser;
use std::{fs::{File, OpenOptions}, io::Write};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    time: u32
}

const DFILE: &str = "/home/zack/biketime.csv";

fn main() {
    let args = Args::parse();
    let mut time = String::new();
    time.push_str(args.time.to_string().as_ref());
    time.push_str("\n");
    match OpenOptions::new().write(true).create(true).append(true).open(DFILE) {
        Ok(mut file) => {
            file.write_all(time.as_ref()).unwrap();
        }
        Err(e) => {

        }
    }
}