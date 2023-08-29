use std::fs::read_to_string;
use std::cmp::max;
use std::cmp::min;

const DFILE: &str = "/home/zack/biketime.csv";


fn main() {
    let mut times: Vec<u32> = Vec::new();
    let mut min_time: u32 = u32::MAX;
    let mut max_time: u32 = u32::MIN;

    for line in read_to_string(DFILE).unwrap().lines() {
        let data: Vec<&str> = line.split(",").collect();
        let time: u32 = data[1].trim().parse().unwrap();
        times.push(time);
    }
    
    for i in 0..times.len() {
        max_time = max(max_time, times[i]);
        min_time = min(min_time, times[i]);
    }

    let sum_time: u32 = times.iter().sum();
    let num_rides: u32 = times.len() as u32;

    let average: f64 = (sum_time / num_rides).into();

    println!("total time:{sum_time}");
    println!("average time:{average}");
    println!("min time:{min_time}");
    println!("max time:{max_time}");
    println!("num rides:{num_rides}");

}

