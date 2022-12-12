use std::{
    collections::{linked_list::CursorMut, HashMap, HashSet, VecDeque},
    fs, vec,
};

fn get_neighbors(grid: Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize, char)> {
    let mut neighbors: Vec<(usize, usize, char)> = vec![];
    for (i, j) in vec![
        (x as i32 - 1, y as i32),
        (x as i32 + 1, y as i32),
        (x as i32, y as i32 - 1),
        (x as i32, y as i32 + 1),
    ] {
        if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
            continue;
        }
        let ch = grid[i as usize][j as usize];
        neighbors.push((i as usize, j as usize, ch));
    }
    neighbors
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

fn part_two() {
    //     let input: String = String::from(
    //         r#"Sabqponm
    // abcryxxl
    // accszExk
    // acctuvwj
    // abdefghi"#,
    //     );
    let input = fs::read_to_string("./inputs/day-12").expect("Unable to read file");
    let mut start_coords: Vec<(usize, usize)> = vec![];
    let mut end_coord: (usize, usize) = (0, 0);
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, f)| match f {
                    'S' | 'a' => {
                        start_coords.push((x, y));
                        'a'
                    }
                    'E' => {
                        end_coord = (x, y);
                        'z'
                    }
                    _ => f as char,
                })
                .collect()
        })
        .collect();

    let mut min = u32::MAX;
    // loop through all
    for start_coord in start_coords {
        let length = find_length(start_coord, end_coord, grid.clone());
        println!(
            "start={:?},end={:?},length={}",
            start_coord, end_coord, length
        );
        if length < min {
            min = length
        }
    }
    println!("min={}", min);
    // process_input(input);
}

pub fn run() {
    part_two();
    // part_one();
    // run_sample();
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
                .map(|(y, f)| match f {
                    'S' => {
                        start_coord = (x, y);
                        'a'
                    }
                    'E' => {
                        end_coord = (x, y);
                        'z'
                    }
                    _ => f as char,
                })
                .collect()
        })
        .collect();

    // dijkstra
    let length = find_length(start_coord, end_coord, grid);
    println!("{}", length);
}

fn find_length(
    start_coord: (usize, usize),
    end_coord: (usize, usize),
    grid: Vec<Vec<char>>,
) -> u32 {
    let mut queue = VecDeque::from(vec![(
        start_coord,
        grid[start_coord.0][start_coord.1],
        0, /*length */
    )]);
    let mut visited: HashSet<(usize, usize)> = HashSet::from([start_coord]);
    // println!("end={:?}", end_coord);
    // println!("queue={:?}, visited={:?}", queue, visited.len());
    // println!("{:?}", visited);
    while !queue.is_empty() {
        let (curr_pos, curr_char, length) = queue.pop_front().unwrap();
        // println!("queue={:?}, visited={:?}", queue.len(), visited.len());
        // println!("curr_pos={:?}", curr_pos);

        if curr_pos == end_coord {
            return length;
        }
        // println!(
        //     "neighbors of ({},{},{})={:?}",
        //     curr_pos.0,
        //     curr_pos.1,
        //     curr_char,
        //     get_neighbors(grid.clone(), curr_pos.0, curr_pos.1)
        // );
        for (n_x, n_y, new_char) in get_neighbors(grid.clone(), curr_pos.0, curr_pos.1) {
            if ((new_char as i32 - curr_char as i32) <= 1) && !visited.contains(&(n_x, n_y)) {
                queue.push_back(((n_x, n_y), new_char, length + 1));
                visited.insert((n_x, n_y));
            }
        }
    }
    u32::MAX
}
