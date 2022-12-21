use std::{fs, result};

use itertools::Itertools;

pub fn run() {
    let input = fs::read_to_string("./inputs/day-20").expect("Unable to read file");
    let nums: Vec<i32> = input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();
    let mixed = mixer(nums);
    solve(mixed);
    // run_sample();
}

pub fn run_sample() {
    let start_nums: Vec<i32> = vec![1, 2, -3, 3, -2, 0, 4];
    let mixed = mixer(start_nums);

    let expected_nums = vec![1, 2, -3, 4, 0, 3, -2];
    assert_eq!(expected_nums, mixed);
    solve(mixed);
}

fn mixer(nums: Vec<i32>) -> Vec<i32> {
    let mut mixed = nums.iter().enumerate().collect::<Vec<(usize, &i32)>>();
    for num_tuple in nums.iter().enumerate() {
        let cur_idx = mixed.iter().position(|x| x == &num_tuple).unwrap();
        mixed.remove(cur_idx);
        let new_idx = ((cur_idx as i32) + num_tuple.1).rem_euclid(mixed.len() as i32) as usize;
        if new_idx == 0 {
            // zero remains in same place
            mixed.push(num_tuple);
        } else {
            mixed.insert(new_idx, num_tuple);
        }
    }
    mixed.iter().map(|x| *x.1).collect::<Vec<i32>>()
}

fn solve(nums: Vec<i32>) {
    let pos_zero = nums.iter().position(|x| x == &0).unwrap();
    println!("nums.len() = {:?}", nums.len());
    println!("pos_zero = {:?}", pos_zero);
    let arr_iter = nums.iter().cycle().skip(pos_zero).take(3001);
    let thousandth = arr_iter.clone().nth(1000).copied();
    let two_thousandth: Option<i32> = arr_iter.clone().nth(2000).copied();
    let three_thousandth: Option<i32> = arr_iter.clone().nth(3000).copied();
    println!("thousandth = {:?}", thousandth);
    println!("two_thousandth = {:?}", two_thousandth);
    println!("three_thousandth = {:?}", three_thousandth);
    println!(
        "sum={:?}",
        thousandth.unwrap() + two_thousandth.unwrap() + three_thousandth.unwrap()
    )
}
