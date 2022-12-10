#![feature(array_windows)]

use anyhow::{bail, Error};
use chrono::{NaiveDate, Utc};
use chrono_tz::America::New_York;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let day;
    if args.len() < 2 {
        let today = Utc::now().with_timezone(&New_York).date_naive();
        let start_date = NaiveDate::from_ymd_opt(2022, 12, 01).unwrap();
        day = (today - start_date).num_days() + 1;
        println!("Day not provided, picking default day {}", day)
    } else {
        day = args[1].parse::<i64>().unwrap();
    }
    match day {
        1 => {
            day1::run();
            Ok(())
        }
        2 => {
            day2::run();
            Ok(())
        }
        3 => {
            day3::run();
            Ok(())
        }
        4 => {
            day4::run();
            Ok(())
        }
        5 => {
            day5::run();
            Ok(())
        }
        6 => {
            day6::run();
            Ok(())
        }
        7 => {
            day7::run();
            Ok(())
        }
        8 => {
            day8::run();
            Ok(())
        }
        9 => {
            day9::run();
            Ok(())
        }
        _ => bail!("not done yet"),
    }
}
