use std::{collections::HashMap, fs};

use itertools::Itertools;

pub fn run() {
    // part_one();
    part_two();
}

fn part_one() {
    let input = fs::read_to_string("./inputs/day-14").expect("Unable to read file");
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let max_depth = parse_rock_groups(input.as_str(), &mut map);
    let mut total = 0;
    while !simulate_grain(max_depth, &mut map, (500, 0), false) {
        total += 1;
    }
    println!("{}", total);
}

fn part_two() {
    let input = fs::read_to_string("./inputs/day-14").expect("Unable to read file");
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let max_depth = parse_rock_groups(input.as_str(), &mut map);
    let mut total = 0;
    while !simulate_grain(max_depth, &mut map, (500, 0), true) {
        total += 1;
    }
    println!("{}", total + 1);
}

fn run_sample() {
    let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    //  Vec<Vec<(i32, i32)>>
    // get something like [[(498, 4), (498, 6), (496, 6)], [(503, 4), (502, 4), (502, 9), (494, 9)]]
    let max_depth = parse_rock_groups(input, &mut map);
    let mut total = 0;
    while !simulate_grain(max_depth, &mut map, (500, 0), false) {
        total += 1;
    }
    println!("{}", total);
}

fn parse_rock_groups(input: &str, map: &mut HashMap<(i32, i32), char>) -> i32 {
    let rock_groups = input
        .split("\n")
        .map(|line| {
            line.split("->")
                .map(|x| {
                    x.split(",")
                        .map(|y| y.trim().parse::<i32>().unwrap())
                        .collect_tuple::<(_, _)>()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    let max_depth = rock_groups
        .clone()
        .into_iter()
        .flatten()
        .map(|(_, y)| y)
        .max()
        .unwrap();
    println!("max depth={}", max_depth);

    // initialize an empty hashmap of 1000x1000
    for x in 0..1000 {
        for y in 0..1000 {
            let _p = map.insert((x, y), '.');
        }
    }

    for rock in &rock_groups {
        let mut start = rock.get(0).unwrap();
        for rock_cell in rock.iter() {
            update_grid(map, start, rock_cell);
            start = &rock_cell;
        }
    }

    return max_depth;
}

fn update_grid(map: &mut HashMap<(i32, i32), char>, start: &(i32, i32), end: &(i32, i32)) {
    let mut current_cell = *start;
    map.insert(*start, '#');
    while current_cell != *end {
        // signum returns +1,0 or -1 depending on whether it's higher or lower
        current_cell.0 = current_cell.0 + (end.0 - start.0).signum();
        current_cell.1 = current_cell.1 + (end.1 - start.1).signum();
        // println!(
        //     "current_cell=>{:?},start={:?},end={:?}",
        //     current_cell, start, end
        // );
        map.insert(current_cell, '#');
    }
}

fn simulate_grain(
    depth: i32,
    map: &mut HashMap<(i32, i32), char>,
    start: (i32, i32),
    part_two: bool,
) -> bool {
    let mut cur = start;
    loop {
        let mut next = (cur.0, cur.1 + 1);
        let char_at_next = *map.get(&next).unwrap();
        // println!(
        //     "cur={:?},next={:?},char_at_next={}",
        //     cur, next, char_at_next
        // );
        if *map.get(&next).unwrap() != '.' {
            // go left
            next.0 = cur.0 - 1;
            if *map.get(&next).unwrap() != '.' {
                // go right
                next.0 = cur.0 + 1;
                if *map.get(&next).unwrap() != '.' {
                    if cur == (500, 0) {
                        return true;
                    }
                    map.insert(cur, 'o');
                    return false;
                }
            }
        }
        cur = next;
        if part_two {
            if cur.1 == depth + 1 {
                map.insert(cur, 'o');
                return false;
            }
        } else {
            if cur.1 > depth {
                return true;
            }
        }
    }
}
