use chrono::DateTime;
use chrono::Datelike;
use chrono::Utc;
use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::exit;

/// biketime - keep track of your rides
#[derive(Parser, Debug)]
struct Args {
    /// the duration of the ride in minutes
    #[arg(short, long)]
    time: u32,
}

fn main() {
    let time: u32 = Args::parse().time;
    if time < 1 {
        println!("parameter time must be > 0.");
        exit(0);
    }

    let dfile: &Path = Path::new("/home/zack/biketime.csv");
    let current_date: DateTime<Utc> = Utc::now();

    let mut line = String::new();
    line.push_str(&current_date.year().to_string());
    line.push_str("-");
    line.push_str(&current_date.month().to_string());
    line.push_str("-");
    line.push_str(&current_date.day().to_string());
    line.push_str(",");
    line.push_str(&time.to_string());
    line.push_str("\n");

    match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(dfile)
    {
        Ok(mut file) => match file.write_all(line.as_ref()) {
            Ok(()) => {
                println!("'{}' written to '{:?}'.", line.trim(), dfile);
            }
            Err(e) => {
                println!("{e}");
                exit(0);
            }
        },
        Err(e) => {
            println!("{e}");
            exit(0);
        }
    }
}