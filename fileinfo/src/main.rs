use clap::Parser;
use std::fs;
use std::fs::Metadata;
use std::io;

#[derive(Parser, Debug)]

struct Args {
    #[arg(short, long)]
    filename: String
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    
    match get_info(args.filename) {
        Ok(info) => {
            println!("{:?}", info);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn get_info(filename: String) -> io::Result<Metadata> {
    fs::metadata(filename)
}
