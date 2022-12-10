use std::{mem::replace, collections::HashMap};

fn test_sample(){
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
   
        run_processor(instructions);
    // println!("{}", registerX)
}

fn run_processor(instructions: Vec<(&str, i32)>){
    let mut curr_cycle = 0u32;
    let mut registerX: i32 = 1;
    let mut cur_idx: usize = 0;
    let mut processor: Vec<(&str,i32, u32)> = vec![];
    let mut signals: HashMap<u32, i32> = HashMap::new();
    while cur_idx < instructions.len(){
    // for (cmd, amt) in instructions {
        match curr_cycle{
            20 | 60 | 100 | 140 | 180 | 220 => {
                signals.insert(curr_cycle, registerX);
                println!("[{}]X:{}", curr_cycle, registerX )
            },
            _ => {}
        }
        if processor.len() > 0 {
            let &mut curr_processing = &mut processor[0];
            let (cmd, amt, left_cycles) = curr_processing;
            if left_cycles == 0 {
                processor.pop();
                cur_idx+=1;
            }
            match cmd {
                "noop" => {continue;},
                "addx" => {
                    // println!("cmd: '{}', cycles: {}", cmd, left_cycles);
                    if left_cycles > 0 {
                        // let _got = replace(&mut stacks[from_idx as usize], from_stack.to_string());
                        let _got = replace(&mut processor[0], (cmd, amt, left_cycles-1));
                    }else {
                        registerX += amt;
                    }
                },
                &_ => todo!(),
            }
        }
        curr_cycle+=1;
        if processor.len() == 0 {
            let (next_cmd, next_amt) = instructions[cur_idx];
            let required_cycles = match next_cmd{
                "noop" => 0,
                "addx" => 1,
                _ => panic!("unknown cmd {}", next_cmd),
            };
            processor.push((next_cmd, next_amt, required_cycles));
            // println!("here [{}] {}, register-X:{}", curr_cycle, next_amt, registerX);
        }
    }
    let mut sum = 0i32;
    for (key, value) in signals.iter(){
        sum += *key as i32*value;
    }
    println!("{}", sum);
}

fn part_one(){
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-10");
    let instructions = lines.iter().map(|line| {
        let w: Vec<&str> = line.split_whitespace().collect();
        if w.len() == 1 {
            (w[0], 0)
        } else {
            (w[0], w[1].parse::<i32>().unwrap())
        }
    })
    .collect();
    run_processor(instructions);
}

pub fn run() {
    // test_sample();
    part_one();
}
