use std::fs;

pub fn run() {
    let input = fs::read_to_string("./inputs/day-20").expect("Unable to read file");
    let mut nums: Vec<i32> = input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();
    mixer(&mut nums);
    solve(nums);
    // run_sample();
}

pub fn run_sample() {
    let mut start_nums: Vec<i32> = vec![1, 2, -3, 3, -2, 0, 4];
    mixer(&mut start_nums);

    let nums = vec![1, 2, -3, 4, 0, 3, -2];
    assert_eq!(nums, start_nums);
    solve(start_nums);
}

fn mixer(start_nums: &mut Vec<i32>) {
    for x in start_nums.clone() {
        let num: i32 = x;
        let cur_idx = start_nums.iter().position(|x| x == &num).unwrap();
        let new_idx = match num.signum() {
            1 => (cur_idx + num as usize) as usize % start_nums.len(),
            -1 => (start_nums.len() - num.abs() as usize % start_nums.len()),
            // 1 | -1 => (cur_idx as i32 + num as i32).rem_euclid(start_nums.len() as i32) as usize,
            // -1 => (cur_idx as i32 + num as i32).rem_euclid(start_nums.len() as i32) as usize,
            0 => continue,
            _ => panic!("impossible"),
        };
        // println!("cur_idx = {:?}, new_idx={:?}", cur_idx, new_idx);
        // println!("start_nums = {:?}, num = {:?}", start_nums, num);
        start_nums.remove(cur_idx);
        start_nums.insert(new_idx, num);
        // println!("start_nums = {:?}, num = {:?}", start_nums, num);
    }
    println!("start_nums = {:?}", start_nums);
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
