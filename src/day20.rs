use std::{fs, result};

use itertools::Itertools;

pub fn run() {
    let input = fs::read_to_string("./inputs/day-20").expect("Unable to read file");
    // part_one(input);
    part_two(input);
    // run_sample();
}

fn part_one(input: String) {
    let nums: Vec<i64> = input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();
    let mixed = mixer(nums, 1, 1);
    solve(mixed);
}

fn part_two(input: String) {
    let nums: Vec<i64> = input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();
    let mixed = mixer(nums, 811589153, 10);
    solve(mixed);
}

pub fn run_sample() {
    let start_nums: Vec<i64> = vec![1, 2, -3, 3, -2, 0, 4];
    let mixed = mixer(start_nums, 1, 1);

    let expected_nums = vec![1, 2, -3, 4, 0, 3, -2];
    assert_eq!(expected_nums, mixed);
    solve(mixed);
}

fn mixer(nums: Vec<i64>, decryption_key: i64, rounds: usize) -> Vec<i64> {
    let mut mixed = nums
        .iter()
        .map(|x| *x * decryption_key)
        .enumerate()
        .collect::<Vec<(usize, i64)>>();
    for _ in 0..rounds {
        for num_tuple in nums
            .clone()
            .into_iter()
            .map(|x| x * decryption_key)
            .enumerate()
        {
            // println!("num_tuple = {:?}", num_tuple);
            let cur_idx = mixed
                .clone()
                .into_iter()
                .position(|x| x == num_tuple)
                .unwrap();
            // println!("cur_idx = {:?}", cur_idx);
            mixed.remove(cur_idx);
            let new_idx = ((cur_idx as i64) + num_tuple.1).rem_euclid(mixed.len() as i64) as usize;
            if new_idx == 0 {
                // zero remains in same place
                mixed.push(num_tuple);
            } else {
                mixed.insert(new_idx, num_tuple);
            }
        }
    }
    mixed.iter().map(|x| x.1).collect::<Vec<i64>>()
}

fn solve(nums: Vec<i64>) {
    let pos_zero = nums.iter().position(|x| x == &0).unwrap();
    println!("nums.len() = {:?}", nums.len());
    println!("pos_zero = {:?}", pos_zero);
    let arr_iter = nums.iter().cycle().skip(pos_zero).take(3001);
    let thousandth = arr_iter.clone().nth(1000).copied();
    let two_thousandth: Option<i64> = arr_iter.clone().nth(2000).copied();
    let three_thousandth: Option<i64> = arr_iter.clone().nth(3000).copied();
    println!("thousandth = {:?}", thousandth);
    println!("two_thousandth = {:?}", two_thousandth);
    println!("three_thousandth = {:?}", three_thousandth);
    println!(
        "sum={:?}",
        thousandth.unwrap() + two_thousandth.unwrap() + three_thousandth.unwrap()
    )
}
