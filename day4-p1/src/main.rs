extern crate chrono;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use chrono::{NaiveDate, NaiveDateTime};
use chrono::Timelike;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut entries: Vec<(NaiveDateTime, &str)> = Vec::new();
    let mut guard_counts: HashMap<u32, i64> = HashMap::new();
    let mut guard_minutes: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in contents.lines() {
        let s: Vec<&str> = line.split("]").collect();
        let time = NaiveDateTime::parse_from_str(s[0], "[%Y-%m-%d %H:%M").expect("Time parse err");
        if s[1].contains("#") {
            let s: Vec<&str> = s[1].split(" begins").collect();
            entries.push((time, &s[0][1..]));
        } else {
            entries.push((time, &s[1][1..]));
        }
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut guard_id = 0;
    let mut sleep: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
    let mut wake: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);

    for entry in entries {
        //println!("{:?}", entry);
        match entry.1 {
            "falls asleep" => {
                //println!("sleep");
                sleep = entry.0;
            },
            "wakes up" => {
                //println!("wake");
                wake = entry.0;
                let mut minutes_vec = guard_minutes.entry(guard_id).or_insert(Vec::new());
                for min in (sleep.minute()..wake.minute()) {
                    minutes_vec.push(min);
                }
                let duration = wake - sleep;
                //println!("slept for {} minutes", duration.num_minutes());
                let g = guard_counts.entry(guard_id).or_insert(0);
                *g += duration.num_minutes();
            },
            _ => {
                guard_id = entry.1[7..].parse::<u32>().unwrap();
                //println!("guard {}", guard_id);
                if !guard_counts.contains_key(&guard_id) {
                    guard_counts.insert(guard_id, 0);
                }
                if !guard_minutes.contains_key(&guard_id) {
                    let minutes: Vec<u32> = Vec::new();
                    guard_minutes.insert(guard_id, minutes);
                }
            },
        }
    }

    let mut counts_vec: Vec<(&u32, &i64)> = guard_counts.iter().collect();
    counts_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{:?}", guard_counts);
    //println!("{}", *counts_vec[0].0 * *counts_vec[0].1);
    println!("{} {}", *counts_vec[0].0, *counts_vec[0].1);
    for min in (0..60) {
        println!("{} : {}", min, guard_minutes[counts_vec[0].0].iter().filter(|&n| *n == min).count());
    }

    Ok(())
}
