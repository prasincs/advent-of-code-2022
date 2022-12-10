use std::collections::HashSet;

fn part_two(input: String) {
    for (idx, w) in input
        .chars()
        .collect::<Vec<char>>()
        .array_windows::<14>()
        .enumerate()
    {
        let h: HashSet<&char> = HashSet::from_iter(w.iter());
        println!("{}: {:?}", idx, w);
        if h.len() == 14 {
            println!("{}", idx + 14);
            break;
        }
    }
}

fn part_one(input: String) {
    for (idx, w) in input
        .chars()
        .collect::<Vec<char>>()
        .array_windows::<4>()
        .enumerate()
    {
        if w[0] == w[1]
            || w[0] == w[2]
            || w[0] == w[3]
            || w[1] == w[2]
            || w[1] == w[3]
            || w[2] == w[3]
        {
            continue;
        } else {
            println!("{}", idx + 4);
            break;
        };
    }
}

pub fn run() {
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-6");
    part_one(lines[0].clone());
    part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string());
    part_two(lines[0].clone());
}
