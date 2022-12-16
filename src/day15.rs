use std::{convert::From, fs};

use regex::Regex;

#[derive(Debug)]
struct Pair {
    sensor: (i32, i32),
    beacon: (i32, i32),
}
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RE: regex::Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
}

impl Pair {
    fn distance(&self) -> u32 {
        manhattan_distance(self.sensor, self.beacon)
    }
}

fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> u32 {
    (a.0.abs_diff(b.0)) + (a.1.abs_diff(b.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distances() {
        assert_eq!(7, manhattan_distance((2, 18), (-2, 15)));
    }
}

fn run_sample() {
    let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
    let items = parse_input(input);
    for item in items {
        println!("{:?},dist={}", item, item.distance());
    }
}

fn part_one() {
    let input = fs::read_to_string("./inputs/day-15").expect("Unable to read file");
    let items = parse_input(input.as_str());
    let mut x_coords = (0, 0);
    let mut y_coords = (0, 0);
    for Pair { sensor, beacon } in &items {
        x_coords.0 = std::cmp::min(x_coords.0, std::cmp::min(sensor.0, beacon.0));
        x_coords.1 = std::cmp::max(x_coords.1, std::cmp::max(sensor.0, beacon.0));
        y_coords.0 = std::cmp::min(y_coords.0, std::cmp::min(sensor.1, beacon.1));
        y_coords.1 = std::cmp::max(y_coords.1, std::cmp::max(sensor.1, beacon.1));
    }

    println!("{:?},{:?}", x_coords, y_coords);

    print_pairs(items);
}

fn print_pairs(items: Vec<Pair>) {
    for item in items {
        println!("{:?},dist={}", item, item.distance());
    }
}

pub fn run() {
    part_one();
}

fn parse_input(input: &str) -> Vec<Pair> {
    let items: Vec<Pair> = input
        .split("\n")
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            Pair {
                sensor: (
                    cap[1].parse::<i32>().unwrap(),
                    cap[2].parse::<i32>().unwrap(),
                ),
                beacon: (
                    cap[3].parse::<i32>().unwrap(),
                    cap[4].parse::<i32>().unwrap(),
                ),
            }
        })
        .collect();
    items
}
