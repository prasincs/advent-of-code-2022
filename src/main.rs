#![feature(array_windows)]
#![feature(linked_list_cursors)]
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
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day20;
mod day21;
mod day25;
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
        10 => {
            day10::run();
            Ok(())
        }
        11 => {
            day11::run();
            Ok(())
        }
        12 => {
            day12::run();
            Ok(())
        }
        13 => {
            day13::run();
            Ok(())
        }
        14 => {
            day14::run();
            Ok(())
        }
        15 => {
            day15::run();
            Ok(())
        }
        16 => {
            day16::run();
            Ok(())
        }
        17 => {
            day17::run();
            Ok(())
        }
        18 => {
            day18::run();
            Ok(())
        }
        20 => {
            day20::run();
            Ok(())
        }
        21 => {
            day21::run();
            Ok(())
        }
        25 => {
            day25::run();
            Ok(())
        }
        _ => bail!("not done yet"),
    }
}
