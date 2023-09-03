use std::fs::read_to_string;
use std::cmp::max;
use std::cmp::min;
use std::path::Path;

fn main() {
    let dfile: &Path = Path::new("/home/zack/biketime.csv");
    let mut dates: Vec<String> = Vec::new();
    let mut times: Vec<u32> = Vec::new();
    let mut min_time: u32 = u32::MAX;
    let mut max_time: u32 = u32::MIN;

    for line in read_to_string(dfile).unwrap().lines() {
        let data: Vec<&str> = line.split(",").collect();
        let date: String = data[0].trim().parse().unwrap();
        let time: u32 = data[1].trim().parse().unwrap();
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

    println!("first run:\t{}", dates[0]);
    println!("last run:\t{}", dates[dates.len()-1]);
    println!("total time:\t{sum_time}");
    println!("average time:\t{:.1}", average);
    println!("min time:\t{min_time}");
    println!("max time:\t{max_time}");
    println!("num rides:\t{num_rides}");

}

