use std::fs::read_to_string;
use std::cmp::max;
use std::cmp::min;

const DFILE: &str = "/home/zack/biketime.csv";


fn main() {
    let mut times: Vec<u32> = Vec::new();
    let mut total: u32 = u32::MIN;
    let mut min_dist: u32 = u32::MAX;
    let mut max_dist: u32 = u32::MIN;

    for line in read_to_string(DFILE).unwrap().lines() {
        let data: Vec<&str> = line.split(",").collect();
        let time: u32 = data[1].trim().parse().unwrap();
        times.push(time);
    }
    
    for i in 0..times.len() {
        total += times[i];
        max_dist = max(max_dist, times[i]);
        min_dist = min(min_dist, times[i]);
    }

    let num_rides: u32 = times.len() as u32;

    let average: u32 = total / num_rides;

    println!("total:{total}");
    println!("average:{average}");
    println!("min:{min_dist}");
    println!("max:{max_dist}");
    println!("rides:{num_rides}");

}

