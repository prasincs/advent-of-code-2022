use std::{collections::HashMap, hash::Hash};

#[derive(Copy, Clone)]
enum Direction {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

fn is_visible(
    direction: Direction,
    visible_map: &HashMap<(usize, usize), bool>,
    x: usize,
    y: usize,
    grid: &Vec<Vec<u32>>,
) -> bool {
    let cur_height = grid.get(x).unwrap().get(y).unwrap();
    if visible_map.contains_key(&(x, y)) {
        return true;
    }
    // println!("cur_height: {}", cur_height);
    match direction {
        Direction::TOP => {
            if x == 0 {
                true
            } else {
                for i in (0usize..=x.saturating_sub(1)).rev() {
                    if is_taller_or_eq(grid, i, y, cur_height) {
                        return false;
                    } else {
                        continue;
                    }
                }
                true
            }
        }
        Direction::BOTTOM => {
            let length = grid.len() - 1;
            if x == length {
                true
            } else {
                for i in x + 1..=length {
                    if is_taller_or_eq(grid, i, y, cur_height) {
                        return false;
                    } else {
                        continue;
                    }
                }
                true
            }
        }
        Direction::RIGHT => {
            let width = grid.get(0).unwrap().len() - 1;
            if y == width {
                true
            } else {
                for i in y + 1..=width {
                    if is_taller_or_eq(grid, x, i, cur_height) {
                        return false;
                    } else {
                        continue;
                    }
                }
                true
            }
        }
        Direction::LEFT => {
            if y == 0 {
                true
            } else {
                for i in 0..=y.saturating_sub(1) {
                    if is_taller_or_eq(grid, x, i, cur_height) {
                        return false;
                    } else {
                        continue;
                    }
                }
                true
            }
        }
        _ => false,
    }
}

fn count_visible(direction: Direction, x: usize, y: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0u32;
    let cur_height = grid.get(x).unwrap().get(y).unwrap();
    // println!("cur_height: {}", cur_height);
    match direction {
        Direction::TOP => {
            if x == 0 {
                return 0u32;
            } else {
                for i in (0usize..=x.saturating_sub(1)).rev() {
                    if is_shorter(grid, i, y, cur_height) {
                        count += 1;
                    }
                    if is_taller_or_eq(grid, i, y, cur_height) {
                        count += 1;
                        break;
                    }
                }
            }
            count
        }
        Direction::BOTTOM => {
            let length = grid.len() - 1;
            if x == length {
                return 0u32;
            } else {
                for i in x + 1..=length {
                    if is_shorter(grid, i, y, cur_height) {
                        count += 1;
                    }
                    if is_taller_or_eq(grid, i, y, cur_height) {
                        count += 1;
                        break;
                    }
                }
                count
            }
        }
        Direction::RIGHT => {
            let width = grid.get(0).unwrap().len() - 1;
            if y == width {
                return 0u32;
            } else {
                for i in y + 1..=width {
                    if is_shorter(grid, x, i, cur_height) {
                        count += 1;
                    }
                    if is_taller_or_eq(grid, x, i, cur_height) {
                        count += 1;
                        break;
                    }
                }
                count
            }
        }
        Direction::LEFT => {
            let width = grid.get(0).unwrap().len() - 1;
            if y == 0 {
                return 0u32;
            } else {
                for i in (0usize..=y.saturating_sub(1)).rev() {
                    if is_shorter(grid, x, i, cur_height) {
                        count += 1;
                    }
                    if is_taller_or_eq(grid, x, i, cur_height) {
                        count += 1;
                        break;
                    }
                }
                count
            }
        }
        _ => 0u32,
    }
}

fn is_taller_or_eq(grid: &Vec<Vec<u32>>, x: usize, y: usize, cur_height: &u32) -> bool {
    let elem_height = grid.get(x).unwrap().get(y).unwrap();
    elem_height >= cur_height
}

fn is_shorter(grid: &Vec<Vec<u32>>, x: usize, y: usize, cur_height: &u32) -> bool {
    let elem_height = grid.get(x).unwrap().get(y).unwrap();
    // println!("checking (x={},y={}), elem_height={}", x, y, elem_height);
    elem_height < cur_height
}

fn test_samples() {
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-8-sample");
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|k| k.to_digit(10).unwrap()).collect())
        .collect();
    let mut visible_map: HashMap<(usize, usize), bool> = HashMap::new();

    assert!(!is_visible(Direction::TOP, &visible_map, 1, 3, &grid));
    assert!(!is_visible(Direction::LEFT, &visible_map, 3, 3, &grid));
    assert!(is_visible(Direction::TOP, &visible_map, 0, 0, &grid));
    assert!(is_visible(
        Direction::BOTTOM,
        &visible_map,
        grid.len() - 1,
        0,
        &grid
    ));
    assert!(!is_visible(Direction::BOTTOM, &visible_map, 2, 1, &grid));
    assert!(is_visible(Direction::TOP, &visible_map, 2, 0, &grid));
    assert!(is_visible(Direction::RIGHT, &visible_map, 2, 0, &grid));
}

fn test_scenic_score() {
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-8-sample");
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|k| k.to_digit(10).unwrap()).collect())
        .collect();
    // println!("{}", count_visible(Direction::TOP, 3, 2, &grid));
    assert_eq!(2, count_visible(Direction::TOP, 3, 2, &grid));
    assert_eq!(2, count_visible(Direction::RIGHT, 3, 2, &grid));
    assert_eq!(1, count_visible(Direction::BOTTOM, 3, 2, &grid));
    assert_eq!(2, count_visible(Direction::LEFT, 3, 2, &grid));

    assert_eq!(1, count_visible(Direction::TOP, 1, 2, &grid));
    assert_eq!(2, count_visible(Direction::RIGHT, 1, 2, &grid));
    assert_eq!(2, count_visible(Direction::BOTTOM, 1, 2, &grid));
    assert_eq!(1, count_visible(Direction::LEFT, 1, 2, &grid));
    // part_two(&grid);
}

fn part_one(grid: &Vec<Vec<u32>>) {
    let mut visible_map: HashMap<(usize, usize), bool> = HashMap::new();

    for x in 0..grid.len() {
        for y in 0..grid.get(0).unwrap().len() {
            if [
                Direction::TOP,
                Direction::LEFT,
                Direction::RIGHT,
                Direction::BOTTOM,
            ]
            .iter()
            .any(|direction| is_visible(*direction, &visible_map, x, y, &grid))
            {
                visible_map.insert((x, y), true);
            }
        }
    }
    println!("{}", visible_map.len());
}

fn part_two(grid: &Vec<Vec<u32>>) {
    let mut visible_map: HashMap<(usize, usize), bool> = HashMap::new();
    let mut scenic_scores: Vec<u32> = vec![];
    for x in 0..grid.len() {
        for y in 0..grid.get(0).unwrap().len() {
            let score = [
                Direction::TOP,
                Direction::LEFT,
                Direction::RIGHT,
                Direction::BOTTOM,
            ]
            .iter()
            .map(|direction| count_visible(*direction, x, y, &grid))
            .fold(1, |mult, i| mult * i);
            scenic_scores.push(score);
        }
    }
    println!("{}", scenic_scores.iter().max().unwrap());
}

pub fn run() {
    test_samples();
    test_scenic_score();
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-8");
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|k| k.to_digit(10).unwrap()).collect())
        .collect();

    part_one(&grid);
    part_two(&grid);

    // for k in visible_map.keys(){
    //     println!("{:?}", k);
    // }
    // for x in 0..grid.len() {
    //     for y in 0..grid.get(0).unwrap().len() {
    //         if visible_map.contains_key(&(x,y)){
    //             print!("[{}]",grid.get(x).unwrap().get(y).unwrap())
    //         }else {
    //             print!(" {} ",grid.get(x).unwrap().get(y).unwrap())
    //         }
    //     }
    //     println!()
    // }
}
