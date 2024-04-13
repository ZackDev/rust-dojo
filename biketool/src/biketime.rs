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
    addentry: Option<u32>,

    /// displays the numbered entries
    #[arg(short, long)]
    showentries: bool,

    /// removes the specified entry
    #[arg(short, long)]
    removeentry: Option<u32>,
}

fn main() {
    match Args::parse().addentry {
        Some(t) => {
            if t < 1 {
                println!("parameter time must be > 0.");
                exit(0);
            }
            addentry(Utc::now(), t);
        }
        None => {}
    }

    match Args::parse().showentries {
        true => {
            showentries();
        },
        false => {
            
        },
    };

    match Args::parse().removeentry {
        Some(r) => {
            if r < 1 {
                println!("parameter removeline must be > 0.");
                exit(0);
            }
            removeentry(r);
        }
        None => {}
    };
}

fn addentry(current_date: DateTime<Utc>, time: u32) {
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

fn showentries() {
    let dfile: &Path = Path::new("/home/zack/biketime.csv");
    match read_to_string(dfile) {
        Ok(str) => {
            let mut index = 1;
            for l in str.lines() {
                println!("{index:.3} {l}");
                index += 1;
            }
        }
        Err(e) => {
            println!("{e}");
            exit(0);
        }
    }
}

fn removeentry(linenumber: u32) {
    let dfile: &Path = Path::new("/home/zack/biketime.csv");
    let mut strbuf: String = "".to_string();
    match read_to_string(dfile) {
        Ok(str) => {
            let fi = str.lines();
            if fi.count() < linenumber as usize {
                println!("linenumber is too high.");
                exit(0);
            }
            let mut index = 1;
            for l in str.lines() {
                if index != linenumber {
                    let li = l.to_owned() + "\n";
                    strbuf.push_str(&li);
                }
                index += 1;
            }
        }
        Err(e) => {
            println!("{e}");
            exit(0);
        }
    }

    match OpenOptions::new().write(true).open(dfile) {
        Ok(mut file) => {
            /*
            rewrite file
             */
            let _ = file.set_len(0);
            match file.write(strbuf.as_bytes()) {
                Ok(_) => println!("file rewritten."),
                Err(e) => {
                    println!("{e}");
                    exit(0);
                }
            }
        },
        Err(_) => {}
    }
}
