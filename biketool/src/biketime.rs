use chrono::DateTime;
use chrono::Datelike;
use chrono::Utc;
use clap::Parser;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::exit;

/// biketime - keep track of your rides
#[derive(Parser, Debug)]
struct Args {
    /// the duration of the ride in minutes
    #[arg(short, long)]
    time: Option<u32>,

    /// the line to remove
    #[arg(short, long)]
    removeline: Option<u32>,
}

#[derive(Parser, Debug)]
struct Args2 {}

fn main() {
    match Args::parse().time {
        Some(t) => {
            if t < 1 {
                println!("parameter time must be > 0.");
                exit(0);
            }
            write(Utc::now(), t);
        }
        None => {}
    }

    match Args::parse().removeline {
        Some(r) => {
            if r < 1 {
                println!("parameter removeline must be > 0.");
                exit(0);
            }
            removeline(r);
        }
        None => {}
    };
}
fn write(current_date: DateTime<Utc>, time: u32) {
    let dfile: &Path = Path::new("/home/zack/biketime.csv");

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

fn removeline(linenumber: u32) {
    let dfile: &Path = Path::new("/home/zack/biketime.csv");
    let mut strbuf: String = "".to_string();
    match read_to_string(dfile) {
        Ok(str) => {
            let mut index = 1;
            for l in str.lines() {
                if index != linenumber {
                    let li = l.to_owned() + "\n";
                    strbuf.push_str(&li);
                }
                index += 1;
            }
        }
        Err(_) => todo!(),
    }

    match OpenOptions::new().write(true).open(dfile) {
        Ok(mut file) => {
            /*
            rewrite file
             */
            match file.write(strbuf.as_bytes()) {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
        },
        Err(_) => {}
    }
}
