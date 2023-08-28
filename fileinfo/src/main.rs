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
            println!("created");
            println!("{:?}", info.created());

            println!("accessed");
            println!("{:?}", info.accessed());

            println!("modified");
            println!("{:?}", info.modified());

            println!("type");
            println!("{:?}", info.file_type());

            println!("directory");
            println!("{:?}", info.is_dir());

            println!("file");
            println!("{:?}", info.is_file());

            println!("symlink");
            println!("{:?}", info.is_symlink());

            println!("permissions");
            println!("{:?}", info.permissions());
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn get_info(filename: String) -> io::Result<Metadata> {
    fs::metadata(filename)
}
