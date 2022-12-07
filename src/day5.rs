use std::mem::replace;

fn mover(lines: Vec<String>, reverse: bool){
    // let mut stacks : Vec<Vec<char>> = Vec::with_capacity(9);
    // [P]     [C]         [M]
    // [D]     [P] [B]     [V] [S]
    // [Q] [V] [R] [V]     [G] [B]
    // [R] [W] [G] [J]     [T] [M]     [V]
    // [V] [Q] [Q] [F] [C] [N] [V]     [W]
    // [B] [Z] [Z] [H] [L] [P] [L] [J] [N]
    // [H] [D] [L] [D] [W] [R] [R] [P] [C]
    // [F] [L] [H] [R] [Z] [J] [J] [D] [D]
    //  1   2   3   4   5   6   7   8   9
    let mut stacks: Vec<String> = vec![
        "PDQRVBHF".to_string(),
        "VWQZDL".to_string(),
        "CPRGQZLH".to_string(),
        "BVJFHDR".to_string(),
        "CLWZ".to_string(),
        "MVGTNPRJ".to_string(),
        "SBMVLRJ".to_string(),
        "JPD".to_string(),
        "VWNCD".to_string(),
    ];

    for (i, line) in lines.iter().enumerate() {
        if i > 9 {
            println!("Processing {}", i - 10);
            let cmd: Vec<&str> = line.split(" ").collect();
            println!("{:?}", cmd);
            let num: i32 = cmd.get(1).unwrap().parse().unwrap();
            let from_idx: i32 = cmd.get(3).unwrap().parse::<i32>().unwrap() - 1;
            let to_idx: i32 = cmd.get(5).unwrap().parse::<i32>().unwrap() - 1;
            // println!("{:?}", stacks);
            let read_stacks = stacks.clone();
            let num_scaled: usize = std::cmp::min(num as usize, read_stacks[from_idx as usize].len());
            let (subs, from_stack) = read_stacks[from_idx as usize].split_at(num_scaled);

            let mut to_stack: String = read_stacks[to_idx as usize].to_string();
            println!(
                "starting: {:?}, {:?}, remainder_from: {:?}, to: {:?}",
                read_stacks[from_idx as usize], subs, from_stack, to_stack
            );
            let to_prepend:String = if reverse {
                subs.chars().rev().collect::<String>()
            }else {
                subs.to_string()
            };
            let lst = [to_prepend, to_stack];
            to_stack = lst.join("").to_owned();
            println!("from_stack: {}, to_stack: {}", from_stack, to_stack);
            let _got = replace(&mut stacks[from_idx as usize], from_stack.to_string());
            let _got = replace(&mut stacks[to_idx as usize], to_stack);
        }
    }
    for s in stacks.iter().map(|x| x.chars().nth(0).unwrap_or_default()).collect::<Vec<char>>(){
        print!("{}", s)
    }
    println!()
}


fn part_one(lines: Vec<String>) {
    mover(lines, true)
}

fn part_two(lines: Vec<String>) {
    mover(lines, false)
}

pub fn run() {
    println!("running for day 5");
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-5");
    // part_one(lines.clone());
    part_two(lines.clone());
}
