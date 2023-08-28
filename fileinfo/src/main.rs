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
    
    match get_info(args.filename) {
        Ok(info) => {
            println!("created:{:?}", info.created());
            println!("accessed:{:?}", info.accessed());
            println!("modified:{:?}", info.modified());
            println!("type:{:?}", info.file_type());
            println!("directory:{:?}", info.is_dir());
            println!("file:{:?}", info.is_file());
            println!("symlink:{:?}", info.is_symlink());
            println!("permissions:{:?}", info.permissions());
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn get_info(filename: String) -> io::Result<Metadata> {
    fs::metadata(filename)
}
