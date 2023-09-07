use chrono::DateTime;
use chrono::Datelike;
use chrono::Utc;
use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::exit;

#[derive(Parser, Debug)]
struct Args {
    /// the duration of the ride in minutes
    #[arg(short, long)]
    time: u32,
}

fn main() {
    let args = Args::parse();
    let dfile: &Path = Path::new("/home/zack/biketime.csv");
    let current_time: DateTime<Utc> = Utc::now();
    let mut line = String::new();
    line.push_str(&current_time.year().to_string());
    line.push_str("-");
    line.push_str(&current_time.month().to_string());
    line.push_str("-");
    line.push_str(&current_time.day().to_string());
    line.push_str(",");
    line.push_str(&args.time.to_string());
    line.push_str("\n");
    match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(dfile)
    {
        Ok(mut file) => {
            file.write_all(line.as_ref()).unwrap();
        }
        Err(e) => {
            println!("{e}");
            exit(0);
        }
    }
}
