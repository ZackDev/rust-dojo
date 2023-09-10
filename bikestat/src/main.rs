use chrono::prelude::*;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
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
    for line in read_to_string(dfile).unwrap().lines() {
        /*
        parse the individual fields of the line
        <year>-<month>-<day>,<time> into
        DateTime<Utc> and u32 and
        push them to the corresponding Vecs dates and times
         */
        let data: Vec<&str> = line.split(",").collect();

        let time: u32;
        match data[1].trim().parse() {
            Ok(v) => {
                time = v;
            }
            Err(e) => {
                println!("{e}");
                exit(0);
            }
        }

        let date_str: String;
        match data[0].trim().parse() {
            Ok(v) => {
                date_str = v;
            }
            Err(e) => {
                println!("{e}");
                exit(0);
            }
        }

        let date_split: Vec<&str> = date_str.split("-").collect();
        if date_split.len() != 3 {
            println!("incorrect date format.");
            exit(0);
        }

        let year: i32;
        match FromStr::from_str(date_split[0]) {
            Ok(v) => {
                year = v;
                if year < 1817 {
                    println!("incorrect year. the first bike was invented in 1817");
                    exit(0);
                }
            }
            Err(e) => {
                println!("{e}");
                exit(0);
            }
        }

        let month: u32;
        match FromStr::from_str(date_split[1]) {
            Ok(v) => {
                month = v;
                if month < 1 || month > 12 {
                    println!("incorrect value for month.");
                    exit(0);
                }
            }
            Err(e) => {
                println!("{e}");
                exit(0);
            }
        }

        let day: u32;
        match FromStr::from_str(date_split[2]) {
            Ok(v) => {
                day = v;
                if day < 1 || day > 31 {
                    println!("incorrect value for day.");
                    exit(0);
                }
            }
            Err(e) => {
                println!("{e}");
                exit(0);
            }
        }

        let date: DateTime<Utc> = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();

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
    let average: f64 = sum_time as f64 / num_rides as f64;

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
    println!("total time:\t{sum_time}m");
    println!("average time:\t{:.1}m", average);
    println!("min time:\t{min_time}m");
    println!("max time:\t{max_time}m");
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
    let mut frequency_str = String::new();
    let mut date_iter = dates[0];
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
