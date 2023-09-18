use chrono::prelude::*;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use clap::Parser;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    options: Option<String>
}

fn main() {
    let options: String;
    match Args::parse().options {
        None => {
            options = "c1noa=rf".to_string();
        }
        Some(o) => {
            options = o;
        }
    }

    let mut dates: Vec<DateTime<Utc>> = Vec::new();
    let mut times: Vec<u32> = Vec::new();

    let dfile: &Path = Path::new("/home/zack/biketime.csv");
    
    let file_content = read_to_string(dfile).unwrap();
    if file_content.len() < 1 {
        println!("no data found in biketime.csv");
        exit(0);
    }
    for line in file_content.lines() {
        /*
        regex the line

        parse the individual fields of the line <year>-<month>-<day>,<time> into DateTime<Utc> and u32 and
        push them to the corresponding Vecs dates and times
        
        regex and parsing errors result in starting the next loop
         */

        // matches lines like [0-9999]-[1-12]-[1-31],[1-x] where x is any integer
        let is_match = Regex::new(r"^(\d|[1-9]\d|[1-9]\d\d|[1-9]\d\d\d)-([1-9]|1[0-2])-([1-9]|[1-2]\d|3[0-1]),([1-9]|[1-9]\d+)$").unwrap().is_match(line);

        if is_match == false {
            continue
        }

        let data: Vec<&str> = line.split(",").collect();

        let time: u32;
        match data[1].trim().parse() {
            Ok(v) => {
                time = v;
            }
            Err(_) => {
                continue
            }
        }

        let date_str: String;
        match data[0].trim().parse() {
            Ok(v) => {
                date_str = v;
            }
            Err(_) => {
                continue
            }
        }

        let date_split: Vec<&str> = date_str.split("-").collect();

        let year: u16;
        match FromStr::from_str(date_split[0]) {
            Ok(v) => {
                year = v;
            }
            Err(_) => {
                continue
            }
        }

        let month: u8;
        match FromStr::from_str(date_split[1]) {
            Ok(v) => {
                month = v;
            }
            Err(_) => {
                continue
            }
        }

        let day: u8;
        match FromStr::from_str(date_split[2]) {
            Ok(v) => {
                day = v;
            }
            Err(_) => {
                continue
            }
        }

        let date: DateTime<Utc> = Utc.with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0).unwrap();

        dates.push(date);
        times.push(time);
    }

    let mut min_time: u32 = u32::MAX;
    let mut max_time: u32 = u32::MIN;
    for i in 0..times.len() {
        /*
        iterate over Vec times and determine max and min cycling times
        initial compare vs u32::MIN and u32::MAX
         */
        max_time = max(max_time, times[i]);
        min_time = min(min_time, times[i]);
    }

    let sum_time: u32 = times.iter().sum();
    let num_rides: u32 = times.len() as u32;
    let average: f32 = sum_time as f32 / num_rides as f32;

    let current_date = Utc::now();
    let current_ymd_date = Utc
        .with_ymd_and_hms(
            current_date.year(),
            current_date.month(),
            current_date.day(),
            0,
            0,
            0,
        )
        .unwrap();
    let freq_str: String = dates_to_frequency_str(dates.clone());

    /*
    print stats to stdout
     */
    
    for c in options.chars() {
        if c == 'c' {
            println!(
                "current date:\t{}-{}-{}",
                current_ymd_date.year(),
                current_ymd_date.month(),
                current_ymd_date.day()
            );
        }
    
        if c == '1' {
            println!(
                "first run:\t{}-{}-{}",
                dates[0].year(),
                dates[0].month(),
                dates[0].day()
            );
        }
    
        if c == 'n' {
            println!(
                "last run:\t{}-{}-{}",
                dates[dates.len() - 1].year(),
                dates[dates.len() - 1].month(),
                dates[dates.len() - 1].day()
            );
        }
    
        if c == 'o' {
            println!("total time:\t{sum_time}");
        }
    
        if c == 'a' {
            println!("average time:\t{:.1}", average);
        }
    
        if c == '=' {
            println!("min time:\t{min_time}");
            println!("max time:\t{max_time}");
        }
    
        if c == 'r' {
            println!("num rides:\t{num_rides}");
        }
    
        if c == 'f' {
            println!("frequency:\t{}", freq_str);
        }
    }



}

fn dates_to_frequency_str(dates: Vec<DateTime<Utc>>) -> String {
    /*
    determine cycling trips per day over
    |first entry|--->|last entry|--->|current date|
    one character per day
    _   no trip
    .   one trip
    :   two+ trips
     */
    let mut f_str: String = String::new();
    let mut d: DateTime<Utc> = dates[0];
    let c: DateTime<Utc> = Utc::now();
    let n: Duration = Duration::days(1);
    while d <= c {
        let f = dates.iter().filter(|&x| *x == d).count();
        if f == 0 {
            f_str.push('_');
        } else if f == 1 {
            f_str.push('.');
        } else if f >= 2 {
            f_str.push(':');
        }
        d += n;
    }
    return f_str;
}
