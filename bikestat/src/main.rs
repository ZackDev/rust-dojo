use std::fs::read_to_string;
use std::cmp::max;
use std::cmp::min;
use std::path::Path;
use std::str::FromStr;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use chrono::prelude::*;

fn main() {
    let dfile: &Path = Path::new("/home/zack/biketime.csv");
    let mut dates: Vec<DateTime<Utc>> = Vec::new();
    let mut times: Vec<u32> = Vec::new();
    let mut min_time: u32 = u32::MAX;
    let mut max_time: u32 = u32::MIN;

    for line in read_to_string(dfile).unwrap().lines() {
        let data: Vec<&str> = line.split(",").collect();
        let time: u32 = data[1].trim().parse().unwrap();
        let date_str: String = data[0].trim().parse().unwrap();
        let date_split: Vec<&str> = date_str.split("-").collect();
        let year: i32 = FromStr::from_str(date_split[0]).unwrap();
        let month: u32 = FromStr::from_str(date_split[1]).unwrap();
        let day: u32 = FromStr::from_str(date_split[2]).unwrap();
        let date = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
        dates.push(date);
        times.push(time);
    }
    
    for i in 0..times.len() {
        max_time = max(max_time, times[i]);
        min_time = min(min_time, times[i]);
    }

    let sum_time: u32 = times.iter().sum();
    let num_rides: u32 = times.len() as u32;
    let average: f64 = sum_time as f64 / num_rides as f64;   
    let freq_str: String = frequency_to_string(dates.clone());
    
    println!("first run:\t{}-{}-{}", dates[0].year(), dates[0].month(), dates[0].day());
    println!("last run:\t{}-{}-{}", dates[dates.len()-1].year(), dates[dates.len()-1].month(), dates[dates.len()-1].day());
    println!("total time:\t{sum_time}");
    println!("average time:\t{:.1}", average);
    println!("min time:\t{min_time}");
    println!("max time:\t{max_time}");
    println!("num rides:\t{num_rides}");
    println!("frequency:\t{}", freq_str);

}

fn frequency_to_string(dates: Vec<DateTime<Utc>>) -> String {
    let mut frequency_str = String::new();
    let mut date_iter = dates[0];
    let current_date = Utc::now();
    let current_ymd_date = Utc.with_ymd_and_hms(current_date.year(), current_date.month(), current_date.day(), 0, 0, 0).unwrap();
    while date_iter <= current_ymd_date {
        let frequency = dates.iter().filter(|&x| *x == date_iter).count();
        if frequency == 0 {
            frequency_str.push('_');
        }
        else if frequency == 1 {
            frequency_str.push('.');
        }
        else if frequency == 2 {
            frequency_str.push(':');
        }
        else if frequency > 2 {
            frequency_str.push('|');
        }
        date_iter = date_iter + Duration::days(1);
    }
    return frequency_str;
}