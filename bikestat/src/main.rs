use chrono::prelude::*;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;

fn main() {
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
    let freq_str: String = frequency_to_string(dates.clone(), current_ymd_date);

    /*
    print stats to stdout
     */
    println!(
        "current date:\t{}-{}-{}",
        current_ymd_date.year(),
        current_ymd_date.month(),
        current_ymd_date.day()
    );
    println!(
        "first run:\t{}-{}-{}",
        dates[0].year(),
        dates[0].month(),
        dates[0].day()
    );
    println!(
        "last run:\t{}-{}-{}",
        dates[dates.len() - 1].year(),
        dates[dates.len() - 1].month(),
        dates[dates.len() - 1].day()
    );
    println!("total time:\t{sum_time}");
    println!("average time:\t{:.1}", average);
    println!("min time:\t{min_time}");
    println!("max time:\t{max_time}");
    println!("num rides:\t{num_rides}");
    println!("frequency:\t{}", freq_str);
}

fn frequency_to_string(dates: Vec<DateTime<Utc>>, current_date: DateTime<Utc>) -> String {
    /*
    determine cycling trips per day over
    |first entry|--->|last entry|--->|current date|
    one character per day
    _   no trip
    .   one trip
    :   two+ trips
     */
    let mut frequency_str: String = String::new();
    let mut date_iter: DateTime<Utc> = dates[0];
    while date_iter <= current_date {
        let frequency = dates.iter().filter(|&x| *x == date_iter).count();
        if frequency == 0 {
            frequency_str.push('_');
        } else if frequency == 1 {
            frequency_str.push('.');
        } else if frequency >= 2 {
            frequency_str.push(':');
        }
        date_iter = date_iter + Duration::days(1);
    }
    return frequency_str;
}
