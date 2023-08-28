use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]

struct Args {
    #[arg(short, long)]
    filename: String
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    let meta = fs::metadata(args.filename);
    println!("{:?}", meta);
}

