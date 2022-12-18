use std::{
    collections::{HashSet, VecDeque},
    fs,
    ops::RangeInclusive,
};

fn neighbors(cube: (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        (cube.0 + 1, cube.1, cube.2),
        (cube.0 - 1, cube.1, cube.2),
        (cube.0, cube.1 + 1, cube.2),
        (cube.0, cube.1 - 1, cube.2),
        (cube.0, cube.1, cube.2 + 1),
        (cube.0, cube.1, cube.2 - 1),
    ]
}

pub fn run() {
    //run_sample();
    // part_one();
    part_two();
}

fn part_one() {
    let input = fs::read_to_string("./inputs/day-18").expect("Unable to read file");
    let cubes = parse_input(input.as_str());
    part_one_solve(cubes)
}

fn part_two() {
    let input = fs::read_to_string("./inputs/day-18").expect("Unable to read file");
    let cubes = parse_input(input.as_str());
    part_two_solve(cubes)
}

fn run_sample() {
    let input = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;
    let cubes = parse_input(input);
    part_one_solve(cubes);
}

fn parse_input(input: &str) -> Vec<(i32, i32, i32)> {
    let cubes: Vec<(i32, i32, i32)> = input
        .split("\n")
        .map(|line| {
            let items: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            (items[0], items[1], items[2])
        })
        .collect();
    cubes
}

fn part_one_solve(cubes: Vec<(i32, i32, i32)>) {
    let mut faces = 0;
    let mut cubes_set = HashSet::new();
    for cube in cubes {
        faces += 6;
        cubes_set.insert(cube);
        for neighbor in neighbors(cube) {
            if cubes_set.contains(&neighbor) {
                faces -= 2;
            }
        }
    }
    println!("faces={}", faces);
}

struct BoundingBox {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    z_range: RangeInclusive<i32>,
}

impl BoundingBox {
    fn from_cubes(cubes: &Vec<(i32, i32, i32)>) -> Self {
        let min_x = cubes.iter().map(|x| x.0).min().unwrap();
        let max_x = cubes.iter().map(|x| x.0).max().unwrap();
        let min_y = cubes.iter().map(|x| x.1).min().unwrap();
        let max_y = cubes.iter().map(|x| x.1).max().unwrap();
        let min_z = cubes.iter().map(|x| x.2).min().unwrap();
        let max_z = cubes.iter().map(|x| x.2).max().unwrap();
        let x_range = min_x - 1..=max_x + 1;
        let y_range = min_y - 1..=max_y + 1;
        let z_range = min_z - 1..=max_z + 1;
        Self {
            x_range,
            y_range,
            z_range,
        }
    }

    fn start_point(&self) -> (i32, i32, i32) {
        (
            *self.x_range.start(),
            *self.y_range.start(),
            *self.z_range.start(),
        )
    }

    fn cube_within_range(&self, cube: (i32, i32, i32)) -> bool {
        self.x_range.contains(&cube.0)
            && self.y_range.contains(&cube.1)
            && self.z_range.contains(&cube.2)
    }
}

fn part_two_solve(cubes: Vec<(i32, i32, i32)>) {
    let bounding_box = BoundingBox::from_cubes(&cubes);
    let mut known_cubes = HashSet::new();
    for cube in cubes.iter() {
        known_cubes.insert(cube);
    }

    let mut touching_water = HashSet::new();
    let mut queue = VecDeque::new();
    let start = bounding_box.start_point();
    queue.push_back(start);
    while let Some(cube) = queue.pop_front() {
        if bounding_box.cube_within_range(cube) {
            for neighbor in neighbors(cube) {
                if !known_cubes.contains(&neighbor) && !touching_water.contains(&neighbor) {
                    queue.push_back(neighbor);
                    touching_water.insert(neighbor);
                }
            }
        }
    }

    let faces: usize = cubes
        .iter()
        .map(|cube| {
            neighbors(*cube)
                .iter()
                .filter(|x| touching_water.contains(x))
                .count()
        })
        .sum();
    println!("faces touching water= {}", faces);
}
