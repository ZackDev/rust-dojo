use std::fs::read_to_string;

const DFILE: &str = "/home/zack/biketime.csv";


fn main() {
    let mut times: Vec<u32> = Vec::new();
    let mut total: u32 = u32::MIN;
    let mut min: u32 = u32::MAX;
    let mut max: u32 = u32::MIN;
    let mut num_rides: u32 = u32::MIN;

    for line in read_to_string(DFILE).unwrap().lines() {
        let data: Vec<&str> = line.split(",").collect();
        let time: u32 = data[1].trim().parse().unwrap();
        times.push(time);
    }
    
    for time in times {
        num_rides += 1;
        total += time;
        if time > max {
            max = time;
        }
        if time < min {
            min = time;
        }
    }

    let average = total / num_rides;

    println!("total:{total}");
    println!("average:{average}");
    println!("min:{min}");
    println!("max:{max}");
    println!("rides:{num_rides}");

}

