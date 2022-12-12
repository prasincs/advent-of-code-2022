use std::{collections::HashMap, fs, vec};

fn get_neighbors(grid: Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize, char)> {
    // println!("neighbors for {:?}", (x, y, grid[x][y]));
    let mut neighbors: Vec<(usize, usize, char)> = vec![];
    for i in x as i32 - 1..=x as i32 + 1 {
        for j in y as i32 - 1..=y as i32 + 1 {
            if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
                continue;
            }
            if i as usize == x && j as usize == y {
                continue;
            }
            let ch = grid[i as usize][j as usize];
            neighbors.push((i as usize, j as usize, ch));
        }
    }
    neighbors
}

fn backtrack(
    grid: Vec<Vec<char>>,
    solution: Vec<(usize, usize, char)>,
    goal: (usize, usize),
    x: usize,
    y: usize,
) -> Option<Vec<(usize, usize, char)>> {
    if solution.len() % 10 == 0 {
        println!(
            "backtracking x={},y={},goal={:?},solution={:?}",
            x,
            y,
            goal,
            solution.len()
        );
    }

    let cur_char = grid[x][y];
    if solution.len() > 5000 {
        panic!("overflow, solution too long = {}", solution.len());
    }
    let mut solution2 = Vec::from(solution);
    solution2.push((x, y, cur_char));
    if is_goal(x, y, goal) {
        return Some(solution2.to_vec());
    } else {
        let neighbors = get_neighbors(grid.clone(), x, y);
        // println!(
        //     "neighbors of (x={},y={}, ch={}) = {:?} ",
        //     x, y, cur_char, neighbors
        // );
        for (n_x, n_y, _) in neighbors {
            if valid_move(grid.clone(), solution2.clone(), cur_char, n_x, n_y) {
                let sol_opt = backtrack(grid.clone(), solution2.clone(), goal, n_x, n_y);
                match sol_opt {
                    Some(solution) => return Some(solution.to_vec()),
                    None => continue,
                }
            }
        }
    }
    None
}

fn is_goal(x: usize, y: usize, goal: (usize, usize)) -> bool {
    x == goal.0 && y == goal.1
}

fn valid_move(
    grid: Vec<Vec<char>>,
    solution: Vec<(usize, usize, char)>,
    cur_char: char,
    row: usize,
    col: usize,
) -> bool {
    let new_char = grid[row][col];
    for (x, y, _) in solution {
        if x == row && y == col {
            // we've already visited this in this solution
            return false;
        }
    }
    if cur_char == 'S' || new_char == 'E' {
        true
    } else {
        (cur_char as u8).abs_diff(new_char as u8) <= 1
    }
}

fn run_sample() {
    let input: String = String::from(
        r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#,
    );
    process_input(input);
}

fn part_one() {
    let input = fs::read_to_string("./inputs/day-12").expect("Unable to read file");
    process_input(input);
}

pub fn run() {
    part_one();
}

fn process_input(input: String) {
    let mut start_coord: (usize, usize) = (0, 0);
    let mut end_coord: (usize, usize) = (0, 0);
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, f)| {
                    match f {
                        'S' => {
                            start_coord = (x, y);
                        }
                        'E' => {
                            end_coord = (x, y);
                        }
                        _ => {}
                    };
                    f as char
                })
                .collect()
        })
        .collect();
    for row in &grid {
        println!("{:?}", row);
    }
    println!("{:?}", end_coord);
    let neighbors = get_neighbors(grid.clone(), start_coord.0, start_coord.1);
    println!("{:?}", neighbors);
    let neighbors = get_neighbors(grid.clone(), 2, 4);
    println!("{:?}", neighbors);
    // lets try some good old backtracking 101
    let solution: Vec<(usize, usize, char)> = vec![];

    let mut map: HashMap<(usize, usize), Option<Vec<(usize, usize, char)>>> = HashMap::new();
    match backtrack(
        grid.clone(),
        solution,
        end_coord,
        start_coord.0,
        start_coord.1,
    ) {
        Some(solution) => {
            println!("{:?}", solution);
            println!("{}", solution.len() - 2 /*removing S and E */);
        }
        None => {
            println!("No Solution")
        }
    }
}
