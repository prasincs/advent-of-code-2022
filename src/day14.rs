use std::{collections::HashMap, iter::Map};

use itertools::Itertools;

pub fn run() {
    let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
    //  Vec<Vec<(i32, i32)>>
    // get something like [[(498, 4), (498, 6), (496, 6)], [(503, 4), (502, 4), (502, 9), (494, 9)]]
    let rock_groups = parse_rock_groups(input);
    println!("{:?}", rock_groups);
}

fn parse_rock_groups(input: &str) -> Vec<Vec<(i32, i32)>> {
    // initialize an empty hashmap of 1000x1000
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    for x in 0..1000 {
        for y in 0..1000 {
            let _p = map.insert((x, y), '.');
        }
    }
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
    rock_groups
}
