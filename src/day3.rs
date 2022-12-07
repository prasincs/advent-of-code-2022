use crate::lines_from_file;
use std::collections::HashSet;

fn get_priority(letter: char) -> u32 {
    match letter {
        'a'..='z' => letter as u32 - 'a' as u32 + 1,
        'A'..='Z' => letter as u32 - 'A' as u32 + 27,
        _ => todo!(),
    }
}

fn find_common(line: &str) -> Option<char> {
    let len = line.len();
    let left_half = &line[0..len / 2];
    let right_half = &line[len / 2..len];
    // println!("{}",left_half);
    // println!("{}",right_half);
    for c in left_half.chars().into_iter() {
        if right_half.contains(c) {
            return Some(c);
        }
    }
    None
}

fn part_one(lines: &Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let common = find_common(line.as_str()).unwrap();
        let priority = get_priority(common);
        sum += priority;
    }
    println!("{}", sum);
}

pub fn tests() {
    assert!(get_priority('A') == 27);
    assert!(get_priority('Z') == 52);
    assert!(get_priority('a') == 1);
    assert!(get_priority('z') == 26);
    assert!(get_priority('r') == 18);

    assert!(find_common("vJrwpWtwJgWrhcsFMMfFFhFp") == Some('p'));
    assert!(find_common("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL") == Some('L'));
    assert!(find_common("PmmdzqPrVvPwwTWBwg") == Some('P'));
    assert!(find_common("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn") == Some('v'));
    assert!(find_common("ttgJtRGJQctTZtZT") == Some('t'));
    assert!(find_common("CrZsJsPPZsGzwwsLwLmpwMDw") == Some('s'));

    assert!(
        find_common_in_group(&[
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ]) == Some('r')
    )
}

fn groups_of_3(lines: &Vec<String>) -> Vec<&[String]> {
    let mut slice_start = 0;
    let mut result = Vec::new();
    for i in 1..lines.len() {
        if i % 3 == 0 {
            result.push(&lines[slice_start..i]);
            slice_start = i;
        }
    }
    if lines.len() > 0 {
        result.push(&&lines[slice_start..]);
    }
    result
}

fn find_common_in_group(group: &[String]) -> Option<char> {
    for g in group[0].chars() {
        let group1_set: HashSet<char> = group[1].chars().collect();
        let group2_set: HashSet<char> = group[2].chars().collect();
        if group1_set.contains(&g) && group2_set.contains(&g) {
            return Some(g);
        }
    }
    None
}

fn part_two(lines: &Vec<String>) {
    // let mut slice_start = 0;
    // let mut result:
    let grp = groups_of_3(&lines);
    let mut sum = 0;
    for g in grp {
        let character = find_common_in_group(g);
        let priority = get_priority(character.unwrap());
        sum += priority;
    }
    println!("{}", sum);
}

pub fn run() {
    let lines = lines_from_file("./inputs/day-3");
    tests();
    part_one(&lines);

    part_two(&lines);
}
