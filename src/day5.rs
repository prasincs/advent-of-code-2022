
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
            let to_prepend = subs.chars().rev().collect::<String>();
            // println!("{:?}", to_prepend);
            let lst = [to_prepend, to_stack];
            // let from_stack = &stacks[from_idx as usize][num..len-1].to_owned();
            // let mut new_str = lst.join("").to_owned();
            // // stacks.get_mut(toIdx as usize).get_or_insert(value)
            // &stacks.insert(toIdx as usize, &new_str.clone());
            // stack_str
            to_stack = lst.join("").to_owned();
            println!("from_stack: {}, to_stack: {}", from_stack, to_stack);
            // new_stack = stacks.spli
            // for (idx, mut_stack) in &mut stacks.iter(){
            // if idx as i32 == from_idx{
            //     mut_stack = stack_str
            // }
            let got = replace(&mut stacks[from_idx as usize], from_stack.to_string());
            let got = replace(&mut stacks[to_idx as usize], to_stack);
            for (i, s) in stacks.iter().enumerate() {
                println!("[{}]{}", i + 1, s);
            }
            println!()
        }
    }
    //println!("{:?}", stacks.get(0));
    for s in stacks.iter().map(|x| x.chars().nth(0).unwrap_or_default()).collect::<Vec<char>>(){
        print!("{}", s)
    }
    println!()
}

pub fn run() {
    println!("running for day 5");
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-5");
    part_one(lines.clone());
    // part_two(lines.clone());
}
