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

/// bikestat - various stats from rides collected with biketime
#[derive(Parser, Debug)]
struct Args {
    /// defines which stats to print (c1noaxrf)
    /// c - current date
    /// 1 - first run
    /// n - last run
    /// o - total time
    /// a - average time
    /// x - min time & max time
    /// r - number of rides
    /// f - frequency
    /// d - duration
    #[arg(short, long, verbatim_doc_comment)]
    options: Option<String>,
}

fn main() {
    let options: String;
    match Args::parse().options {
        None => {
            options = "c1noaxrfd".to_string();
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
            continue;
        }

        let data: Vec<&str> = line.split(",").collect();

        if options.contains('o')
            || options.contains('a')
            || options.contains('x')
            || options.contains('r')
        {
            let time: u32;
            match data[1].trim().parse() {
                Ok(v) => {
                    time = v;
                }
                Err(_) => continue,
            }
            times.push(time);
        }

        if options.contains('1') || options.contains('n') || options.contains('f') {
            let date_str: String;
            match data[0].trim().parse() {
                Ok(v) => {
                    date_str = v;
                }
                Err(_) => continue,
            }

            let date_split: Vec<&str> = date_str.split("-").collect();

            let year: u16;
            match FromStr::from_str(date_split[0]) {
                Ok(v) => {
                    year = v;
                }
                Err(_) => continue,
            }

            let month: u8;
            match FromStr::from_str(date_split[1]) {
                Ok(v) => {
                    month = v;
                }
                Err(_) => continue,
            }

            let day: u8;
            match FromStr::from_str(date_split[2]) {
                Ok(v) => {
                    day = v;
                }
                Err(_) => continue,
            }

            let date: DateTime<Utc> = Utc
                .with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0)
                .unwrap();

            dates.push(date);
        }
    }

    let mut min_time: u32 = u32::MAX;
    let mut max_time: u32 = u32::MIN;
    if options.contains('x') || options.contains('d') {
        for i in 0..times.len() {
            /*
            iterate over Vec times and determine max and min cycling times
            initial compare vs u32::MIN and u32::MAX
             */
            max_time = max(max_time, times[i]);
            min_time = min(min_time, times[i]);
        }
    }

    let mut sum_time: u32 = 0;
    if options.contains('o') || options.contains('a') {
        sum_time = times.iter().sum();
    }

    let mut num_rides: u32 = 0;
    if options.contains('r') || options.contains('a') {
        num_rides = times.len() as u32;
    }

    let mut average: f32 = 0.0;
    if options.contains('a') {
        average = sum_time as f32 / num_rides as f32;
    }

    let current_date = Utc::now();

    let mut freq_str: String = String::new();
    if options.contains('f') {
        freq_str = dates_to_frequency_str(dates.clone());
    }

    let mut dur_str: String = String::new();
    if options.contains('d') {
        dur_str = dates_and_times_to_daily_duration(dates.clone(), times.clone(), max_time);
    }

    /*
    print stats to stdout
     */

    for c in options.chars() {
        if c == 'c' {
            println!(
                "current date:\t{}-{}-{}",
                current_date.year(),
                current_date.month(),
                current_date.day()
            );
        } else if c == '1' {
            println!(
                "first run:\t{}-{}-{}",
                dates[0].year(),
                dates[0].month(),
                dates[0].day()
            );
        } else if c == 'n' {
            println!(
                "last run:\t{}-{}-{}",
                dates[dates.len() - 1].year(),
                dates[dates.len() - 1].month(),
                dates[dates.len() - 1].day()
            );
        } else if c == 'o' {
            println!("total time:\t{sum_time}");
        } else if c == 'a' {
            println!("average time:\t{:.1}", average);
        } else if c == 'x' {
            println!("min time:\t{min_time}");
            println!("max time:\t{max_time}");
        } else if c == 'r' {
            println!("num rides:\t{num_rides}");
        } else if c == 'f' {
            println!("frequency:\t{}", freq_str);
        } else if c == 'd' {
            println!("duration:\t{}", dur_str);
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

fn dates_and_times_to_daily_duration(
    dates: Vec<DateTime<Utc>>,
    times: Vec<u32>,
    max_time: u32,
) -> String {
    let len = dates.len();
    let mut c_date = dates[0];
    let mut d_time = times[0];
    let mut a_times: Vec<u32> = Vec::new();
    let n: Duration = Duration::days(1);

    for i in 1..len {
        if c_date == dates[i] {
            d_time += times[i];
        } else {
            a_times.push(d_time);
            c_date += n;
            while c_date < dates[i] {
                c_date += n;
                a_times.push(0);
            }
            d_time = times[i];
            if i == len - 1 {
                a_times.push(d_time);
            }
        }
    }

    let mut d_str: String = String::new();
    let upper: u32 = max_time * 2 / 3;
    let lower: u32 = max_time * 1 / 3;

    for a in a_times {
        if a < max_time && a >= upper {
            d_str.push('|');
        } else if a < upper && a >= lower {
            d_str.push(':');
        } else if a < lower && a > 0 {
            d_str.push('.');
        } else if a == 0 {
            d_str.push('_');
        } else if a >= max_time {
            d_str.push('!');
        }
    }
    return d_str;
}
