
fn part_one(lines: Vec<String>){
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
    let mut stacks = vec![[
        "PDQRVBHF".chars(),
        "VWQZDL".chars(),
        "CPRGQZLH".chars(),
        "BVJFHDR".chars(),
        "CLWZ".chars(),
        "MVGTNPRJ".chars(),
        "SBMVLRJ".chars(),
        "JPD".chars(),
        "VWNCD".chars(),
    ]];
    for (i, line) in lines.iter().enumerate() {
        // if i < 10 {
        //     println!("{}",line);
        // for (j, ch) in line.chars().enumerate(){
        //     if ch == '[' || ch == ']' || ch.is_whitespace(){
        //         continue
        //     }
        //     // let mut stack = stacks.get(j).unwrap().to_vec();
        //     // stack.push(stack);
        //     // stacks.get_mut(i)
        //         println!("{},{}",j, ch);
        //     if let Some(stack) = stacks.get_mut(j){
        //         stack.push(ch);
        //     }
        // }
    // }

        if i > 10 {
            let cmd: Vec<&str> = line.split(" ").collect();
            println!("{:?}", cmd);
            let num: i32 = cmd.get(1).unwrap().parse().unwrap();
            let fromIdx: i32 = cmd.get(3).unwrap().parse::<i32>().unwrap() -1;
            let toIdx: i32 = cmd.get(3).unwrap().parse::<i32>().unwrap() -1;
        }
    }
}

pub fn run(){
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-5");
    part_one(lines.clone());
    // part_two(lines.clone());
}