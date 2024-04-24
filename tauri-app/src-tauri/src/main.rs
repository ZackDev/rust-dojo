// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::OpenOptions, io::Write};

use chrono::{DateTime, Datelike, Utc};
use homedir::get_my_home;

static FILE_STR: &str = "biketime.csv";

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![addtime])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
