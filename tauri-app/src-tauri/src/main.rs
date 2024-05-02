// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{cmp::{max, min}, collections::HashMap, fs::{read_to_string, OpenOptions}, io::Write, path::Path};

use chrono::{DateTime, Datelike, Duration, TimeDelta, TimeZone, Utc};
use homedir::get_my_home;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

static FILE_STR: &str = "biketime.csv";

#[derive(Serialize, Deserialize, Debug)]
struct FreqStruct {
    dates: Vec<String>,
    frequency: Vec<String>
} 

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn addtime(time: &str) -> String {
    let fpath = get_my_home().unwrap().unwrap().display().to_string() + "/" + FILE_STR;

    let time_u32 :u32;

    match time.parse() {
        Ok(v) => {
            time_u32 = v;
        },
        Err(e) => {
            return format!("{e}");
        }
        
    }

    let current_date: DateTime<Utc> = Utc::now();

    let mut line = String::new();
    line.push_str(&current_date.year().to_string());
    line.push_str("-");
    line.push_str(&current_date.month().to_string());
    line.push_str("-");
    line.push_str(&current_date.day().to_string());
    line.push_str(",");
    line.push_str(&time_u32.to_string());
    line.push_str("\n");

    /*
    write entry string to file
        */
    match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(fpath.clone())
    {
        Ok(mut file) => match file.write_all(line.as_ref()) {
            Ok(()) => {
                format!("'{}' written to '{:?}'.", line.trim(), fpath)
            }
            Err(e) => {
                format!("{e}")
            }
        },
        Err(e) => {
            format!("{e}")
        }
    }
}

#[tauri::command]
fn getstats(stat: char) -> String {
    let fpath = get_my_home().unwrap().unwrap().display().to_string() + "/" + FILE_STR;

    let mut dates: Vec<DateTime<Utc>> = Vec::new();
    let mut times: Vec<u32> = Vec::new();

    let mut file_content = String::new();
    match read_to_string(Path::new(&fpath)) {
        Ok(cstring) => {
            file_content.push_str(&cstring);
        }
        Err(_) => {
            format!("couldn't open {}.", fpath);
        }
    }

    if file_content.len() < 1 {
        format!("no data found in {}.", fpath);
    }
    for line in file_content.lines() {
        /*
        regex the line

        parse the individual fields of the line <year>-<month>-<day>,<time> into DateTime<Utc> and u32,
        and push them to the corresponding Vecs dates and times

        regex and parsing errors skip the current line
         */

        /* matches lines like [0-9999]-[1-12]-[1-31],[1-x] where x is any integer */
        let is_valid = Regex::new(r"^(\d|[1-9]\d|[1-9]\d\d|[1-9]\d\d\d)-([1-9]|1[0-2])-([1-9]|[1-2]\d|3[0-1]),([1-9]|[1-9]\d+)$").unwrap().is_match(line);

        if is_valid == !true {
            continue;
        }

        let data: Vec<&str> = line.split(",").collect();

        /*
        the value time is needed for these options
         */
        if stat == 'o'
            || stat == 'a'
            || stat == 'x'
            || stat == 'r'
            || stat == 'd'
        {
            /* yay, turbofish */
            match data[1].trim().parse::<u32>() {
                Ok(time) => {
                    times.push(time);
                }
                Err(_) => continue
            }
            
        }

        /* these options require the date value */
        if stat == '1'
            || stat == 'n'
            || stat == 'f'
            || stat == 'd'
        {
            let date_str: String;
            match data[0].trim().parse::<String>() {
                Ok(d_str) => {
                    date_str = d_str;
                }
                Err(_) => continue
            }

            let date_split: Vec<&str> = date_str.split("-").collect();

            let year: i32;
            match i32::from_str(date_split[0]) {
                Ok(y) => {
                    year = y;
                }
                Err(_) => continue
            }

            let month: u32;
            match u32::from_str(date_split[1]) {
                Ok(m) => {
                    month = m;
                }
                Err(_) => continue
            }

            let day: u32;
            match u32::from_str(date_split[2]) {
                Ok(d) => {
                    day = d;
                }
                Err(_) => continue
            }

            match Utc.with_ymd_and_hms(year, month, day, 0, 0, 0) {
                chrono::LocalResult::Single(date) => {
                    dates.push(date);
                },
                chrono::LocalResult::None => {
                    continue
                },
                chrono::LocalResult::Ambiguous(_, _) => {
                    continue
                },
            };
        }
    }

    let mut min_time: u32 = u32::MAX;
    let mut max_time: u32 = u32::MIN;
    if stat == 'x' {
        for i in 0..times.len() {
            /*
            iterate over Vec times and determine max and min cycling times
            initial compare:
                max_time <-> u32::MIN
                min_time <-> u32::MAX
             */
            max_time = max(max_time, times[i]);
            min_time = min(min_time, times[i]);
        }
    }

    let mut sum_time: u32 = 0;
    if stat == 'o' || stat == 'a' {
        sum_time = times.iter().sum();
    }

    let mut num_rides: u32 = 0;
    if stat == 'r' || stat == 'a' {
        num_rides = times.len() as u32;
    }

    let mut average: f32 = 0.0;
    if stat == 'a' {
        average = sum_time as f32 / num_rides as f32;
    }
    
    let current_date = Utc::now();

    let mut freq_str: String = String::new();
    if stat == 'f' {
        freq_str = craft_frequency_str(&mut dates);
    }

    let mut dur_str: String = String::new();
    if stat == 'd' {
        dur_str = craft_duration_str(dates.clone(), times.clone());
    }

    if stat == 'c' {
        let s = format!(
            "{}-{}-{}",
            current_date.year(),
            current_date.month(),
            current_date.day()
            );
        return s;
        }
    if stat == '1' {
        let s = format!(
            "{}-{}-{}",
            dates[0].year(),
            dates[0].month(),
            dates[0].day()
            );
        return s;
    }
    if stat == 'n' {
        let s = format!(
            "{}-{}-{}",
            dates[dates.len() - 1].year(),
            dates[dates.len() - 1].month(),
            dates[dates.len() - 1].day()
            );
        return s;
    }
    if stat == 'o' {
        let mut m_multiple_str:String = "minute".to_string();
        let h_str:String = "hours".to_string();
        let d_str:String = "days".to_string();
        
        let minutes = sum_time;
        let hours = sum_time as f32 / 60.0;
        let days = sum_time as f32 / 1440.0;
        
        if minutes > 1 {
            m_multiple_str += "s";
        }
        let s = format!("{minutes} {m_multiple_str} or {hours:.2} {h_str} or {days:.2} {d_str}");
        return s;
    }
    if stat == 'a' {
        let s = format!("{:.1}", average);
        return s;
    }
    if stat == 'x' {
        let s = format!("min:\t{min_time}\tmax:\t{max_time}");
        return s;
    }
    if stat == 'r' {
        let s = format!("{num_rides}");
        return s;
    }
    if stat == 'f' {
        let s = format!("{}", freq_str);
        return s;
    }
    if stat == 'd' {
        let s = format!("{}", dur_str);
        return s;
    }
    else {
        return "".to_string();
    }
}

fn craft_frequency_str(dates: &mut Vec<DateTime<Utc>>) -> String {
    /*
    determine cycling trips per day from first run to current date
     */
    let mut f_struct = FreqStruct { dates: Vec::new(), frequency: Vec::new()};
    let mut h_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut d: DateTime<Utc> = dates[0];
    let c: DateTime<Utc> = Utc::now();
    let n: Duration = TimeDelta::try_days(1).unwrap();
    while d <= c {
        let f = dates.iter().filter(|&date| *date == d).count();
        let d_str = format!("{}-{}-{}", d.year(), d.month(), d.day());
        f_struct.dates.push(d_str);
        f_struct.frequency.push(f.to_string());
        d += n;
    }
    h_map.insert("dates".to_string(), f_struct.dates);
    h_map.insert("frequency".to_string(), f_struct.frequency);
    let ser = serde_json::to_string(&h_map).unwrap();
    return ser;
}

fn craft_duration_str(mut dates: Vec<DateTime<Utc>>, mut times: Vec<u32>) -> String {
    let n: Duration = TimeDelta::try_days(1).unwrap();
    let current_date = Utc::now();
    let mut selected_date = dates[0];

    /*
    normalize times and dates vectors, fill the gaps, expand to current date
    
    NOTE: consumes dates and times
    */
    let mut norm_dates: Vec<DateTime<Utc>> = Vec::new();
    let mut norm_times: Vec<u32> = Vec::new();

    while selected_date <= current_date {
        if !dates.contains(&selected_date) {
            norm_dates.push(selected_date);
            norm_times.push(0);
        }
        while dates.contains(&selected_date) {
            let i = dates
                .iter()
                .position(|&date| date == selected_date)
                .unwrap();
            norm_dates.push(dates[i]);
            norm_times.push(times[i]);
            dates.remove(i);
            times.remove(i);
        }
        selected_date += n;
    }

    /* 
    - accumulate times by given day in vector acc_times
    
    - NOTE: consumes norm_dates and norm_times
    */
    let mut acc_times: Vec<u32> = Vec::new();
    let mut acc_time: u32;
    selected_date = norm_dates[0];

    while selected_date <= current_date {
        acc_time = 0;
        while norm_dates.contains(&selected_date) {
            let i = norm_dates
                .iter()
                .position(|&date| date == selected_date)
                .unwrap();
            acc_time += norm_times[i];
            norm_dates.remove(i);
            norm_times.remove(i);
        }
        selected_date += n;
        acc_times.push(acc_time);
    }

    let mut a_times_clone = acc_times.clone();
    a_times_clone.sort();
    let max_time = a_times_clone[a_times_clone.len() - 1];
    let upper: u32 = max_time * 2 / 3;
    let lower: u32 = max_time / 3;
    let mut d_str: String = String::new();

    for t in acc_times {
        if t == 0 {
            d_str.push('_');
        } else if t < lower && t > 0 {
            d_str.push('.');
        } else if t < upper && t >= lower {
            d_str.push(':');
        } else if t < max_time && t >= upper {
            d_str.push('|');
        } else if t == max_time {
            d_str.push('!');
        }
    }
    return d_str;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![addtime, getstats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
