use std::{collections::HashMap, mem::replace, str::Chars};

fn test_sample() {
    let input: String = String::from(
        r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#,
    );
    // let input = String::from(
    // r#"noop
    // addx 3
    // addx -5"#);
    let instructions: Vec<(&str, i32)> = input
        .split("\n")
        .map(|line| {
            let w: Vec<&str> = line.split_whitespace().collect();
            if w.len() == 1 {
                (w[0], 0)
            } else {
                (w[0], w[1].parse::<i32>().unwrap())
            }
        })
        .collect();

    run_processor(instructions, Some(true));
    // println!("{}", registerX)
}

fn run_processor(instructions: Vec<(&str, i32)>, draw: Option<bool>) {
    let mut curr_cycle = 0u32;
    let mut registerX: i32 = 1;
    let mut cur_idx: usize = 0;
    let mut processor: Vec<(&str, i32, u32)> = vec![];
    let mut signals: HashMap<u32, i32> = HashMap::new();
    let mut screen: Vec<[&str; 40]> = vec![
        [
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".",
        ],
        [
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".",
        ],
        [
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".",
        ],
        [
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".",
        ],
        [
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".",
        ],
        [
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ".", ".", ".", ".", ".", ".",
        ],
    ];
    while cur_idx < instructions.len() {
        match curr_cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                signals.insert(curr_cycle, registerX);
                // println!("[{}]X:{}", curr_cycle, registerX )
            }
            240 => break,
            _ => {
                if draw == Some(true) && curr_cycle > 0 {
                    let curr_end_cycle = curr_cycle - 1;
                    let row: usize = (curr_end_cycle / 40) as usize;
                    let col: usize = (curr_end_cycle % 40) as usize;
                    // println!(
                    //     "[{}]X:{}, row:{}, col:{}",
                    //     curr_end_cycle, registerX, row, col
                    // );

                    // let row_x: usize = (registerX / 40) as usize;
                    let col_x: usize = (registerX % 40) as usize;
                    // println!(
                    //     "[{}] col_x as i32 - col as i32).abs() == {}",
                    //     curr_end_cycle,
                    //     (col_x as i32 - col as i32).abs()
                    // );
                    if (col_x as i32 - col as i32).abs() < 2 {
                        let _got = replace(&mut screen[row][col], "#");
                    } else {
                        let _got = replace(&mut screen[row][col], ".");
                    }
                }
            }
        }
        if processor.len() > 0 {
            let &mut curr_processing = &mut processor[0];
            let (cmd, amt, left_cycles) = curr_processing;
            if left_cycles == 0 {
                processor.pop();
                cur_idx += 1;
            }
            match cmd {
                "noop" => {
                    continue;
                }
                "addx" => {
                    if left_cycles > 0 {
                        let _got = replace(&mut processor[0], (cmd, amt, left_cycles - 1));
                    } else {
                        registerX += amt;
                    }
                }
                &_ => todo!(),
            }
        }
        curr_cycle += 1;
        if processor.len() == 0 {
            let (next_cmd, next_amt) = instructions[cur_idx];
            let required_cycles = match next_cmd {
                "noop" => 0,
                "addx" => 1,
                _ => panic!("unknown cmd {}", next_cmd),
            };
            processor.push((next_cmd, next_amt, required_cycles));
        }
    }
    let mut sum = 0i32;
    for (key, value) in signals.iter() {
        sum += *key as i32 * value;
    }
    match draw {
        Some(true) => {
            for s in screen {
                println!("{}", s.join(""));
            }
        }
        None | Some(false) => println!("sum: {}", sum),
    }
}

fn part_one() {
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-10");
    let instructions: Vec<(&str, i32)> = lines
        .iter()
        .map(|line| {
            let w: Vec<&str> = line.split_whitespace().collect();
            if w.len() == 1 {
                (w[0], 0)
            } else {
                (w[0], w[1].parse::<i32>().unwrap())
            }
        })
        .collect();
    run_processor(instructions, None);
}

fn part_two() {
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-10");
    let instructions: Vec<(&str, i32)> = lines
        .iter()
        .map(|line| {
            let w: Vec<&str> = line.split_whitespace().collect();
            if w.len() == 1 {
                (w[0], 0)
            } else {
                (w[0], w[1].parse::<i32>().unwrap())
            }
        })
        .collect();
    run_processor(instructions, Some(true));
}

pub fn run() {
    part_one();
    part_two();
}
