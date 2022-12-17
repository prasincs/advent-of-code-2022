use std::{
    collections::{HashMap, VecDeque},
    fs,
};

struct Symbol {
    rock_map: Vec<(usize, usize)>,
}

impl Symbol {
    fn from_rock_map(rock_map: Vec<(usize, usize)>) -> Self {
        Self { rock_map }
    }

    fn from(value: Vec<Vec<char>>) -> Self {
        // convert to numbers so that it's easier to compute whether it fits
        let mut rock_map: Vec<(usize, usize)> = vec![];
        for x in 0..value.len() {
            for y in 0..value[x].len() {
                if value[x][y] == '#' {
                    rock_map.push((x, y))
                }
            }
        }
        Self { rock_map }
    }

    fn fits_in_chamber(&self, chamber: &[[u8; 7]], height: usize, width: usize) -> bool {
        // only return true iff it fits all the elements
        self.rock_map
            .iter()
            .all(|(dh, dw)| width + dw < 7 && chamber[height + dh][width + dw] != b'#')
    }
}

fn get_height(map: &[[u8; 7]]) -> usize {
    // get height that has last &[0;7]
    map.iter().position(|row| row == &[0; 7]).unwrap()
}

fn chamber_heights(map: &[[u8; 7]]) -> [usize; 7] {
    let mut heights = [0; 7];
    let h = get_height(map);
    for i in 0..7 {
        heights[i] = (0..h)
            .find(|&x| map[h - x][i] == b'#')
            .unwrap_or(usize::MAX);
    }
    heights
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_symbol() {
        let cross = Symbol::from(vec![
            vec![' ', '#', ' '],
            vec!['#', '#', '#'],
            vec![' ', '#', ' '],
        ]);
        assert_eq!(vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)], cross.rock_map);

        let flat = Symbol::from(vec![vec!['#', '#', '#', '#']]);
        assert_eq!(vec![(0, 0), (0, 1), (0, 2), (0, 3)], flat.rock_map);
    }
}

fn run_sample() {
    let input = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
    part_two_solve(input);
}

fn part_one() {
    let input = fs::read_to_string("./inputs/day-17").expect("Unable to read file");
    part_one_solve(input.as_str());
}

fn part_two() {
    let input = fs::read_to_string("./inputs/day-17").expect("Unable to read file");
    part_two_solve(input.as_str());
}

pub fn run() {
    // run_sample();
    // part_one();
    part_two();
}

fn part_one_solve(input: &str) {
    println!("input len={}", input.len());
    let symbols: [Symbol; 5] = [
        Symbol::from(vec![vec!['#', '#', '#', '#']]),
        Symbol::from(
            /*
            (0,1),(1,0),(1,1),(1,2),(2,1)
            */
            vec![
                vec![' ', '#', ' '],
                vec!['#', '#', '#'],
                vec![' ', '#', ' '],
            ],
        ),
        Symbol::from(vec![
            vec!['#', '#', '#'],
            vec![' ', ' ', '#'],
            vec![' ', ' ', '#'],
        ]),
        Symbol::from(vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']]),
        Symbol::from(vec![vec!['#', '#'], vec!['#', '#']]),
    ];
    for symbol in &symbols {
        println!("symbol.rock_map = {:?}", symbol.rock_map);
    }
    // allocate length of the current
    // make tall enough chamber to fit full run
    let mut chamber_map = [[0u8; 7]; 10000];
    let (mut i, mut tick, mut total_height) = (0, 0, 0);
    // let mut cache = HashMap::new();
    while i < 2022 {
        let symbol = &symbols[i % symbols.len()];
        let (mut h, mut w) = (get_height(&chamber_map) + 3, 2);
        loop {
            // println!("h = {:?}", h);
            // println!("w = {:?}", w);
            match input.as_bytes()[tick % input.len()] {
                b'>' => {
                    if symbol.fits_in_chamber(&chamber_map, h, w + 1) {
                        w += 1;
                    }
                }
                b'<' => {
                    if w != 0 {
                        if symbol.fits_in_chamber(&chamber_map, h, w - 1) {
                            w -= 1;
                        }
                    }
                }
                _ => panic!("invalid input"),
            };
            tick += 1;
            if h == 0 || !symbol.fits_in_chamber(&chamber_map, h - 1, w) {
                break;
            }
            h -= 1;
        }
        // println!("{:?}", symbol.rock_map.clone());
        for (dh, dw) in symbol.rock_map.clone() {
            // println!("dh, dw = {:?}, {:?}", dh, dw);
            // println!("h, w = {:?}, {:?}", h + dh, w + dw);
            chamber_map[h + dh][w + dw] = b'#';
        }

        // print_chamber_map(chamber_map);

        println!("rocks= {} height = {}", i + 1, get_height(&chamber_map));
        println!("========================================");
        i += 1;
    }
}

fn part_two_solve(input: &str) {
    println!("input len={}", input.len());
    let symbols: [Symbol; 5] = [
        Symbol::from(vec![vec!['#', '#', '#', '#']]),
        Symbol::from(
            /*
            (0,1),(1,0),(1,1),(1,2),(2,1)
            */
            vec![
                vec![' ', '#', ' '],
                vec!['#', '#', '#'],
                vec![' ', '#', ' '],
            ],
        ),
        Symbol::from(vec![
            vec!['#', '#', '#'],
            vec![' ', ' ', '#'],
            vec![' ', ' ', '#'],
        ]),
        Symbol::from(vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']]),
        Symbol::from(vec![vec!['#', '#'], vec!['#', '#']]),
    ];
    for symbol in &symbols {
        println!("symbol.rock_map = {:?}", symbol.rock_map);
    }
    // allocate length of the current
    // make tall enough chamber to fit full run
    let mut chamber_map = [[0u8; 7]; 10000];
    let (mut i, mut tick, mut total_height) = (0, 0, 0);
    let mut cache = HashMap::new();
    while i < 1000000000000 {
        let symbol = &symbols[i % symbols.len()];
        let (mut h, mut w) = (get_height(&chamber_map) + 3, 2);
        loop {
            // println!("h = {:?}", h);
            // println!("w = {:?}", w);
            match input.as_bytes()[tick % input.len()] {
                b'>' => {
                    if symbol.fits_in_chamber(&chamber_map, h, w + 1) {
                        w += 1;
                    }
                }
                b'<' => {
                    if w != 0 {
                        if symbol.fits_in_chamber(&chamber_map, h, w - 1) {
                            w -= 1;
                        }
                    }
                }
                _ => panic!("invalid input"),
            };
            tick += 1;
            if h == 0 || !symbol.fits_in_chamber(&chamber_map, h - 1, w) {
                break;
            }
            h -= 1;
        }
        // println!("{:?}", symbol.rock_map.clone());
        for (dh, dw) in symbol.rock_map.clone() {
            // println!("dh, dw = {:?}, {:?}", dh, dw);
            // println!("h, w = {:?}, {:?}", h + dh, w + dw);
            chamber_map[h + dh][w + dw] = b'#';
        }

        // print_chamber_map(chamber_map);

        println!(
            "rocks= {} height = {}",
            i + 1,
            total_height + get_height(&chamber_map)
        );
        println!("========================================");

        let key = (
            i % symbols.len(),
            tick % input.len(),
            chamber_heights(&chamber_map),
        );
        // idea is to find a point where we get a repetition -- if we find it, we can take that index and jump ahead
        // dont need to iterate through rest
        if let Some((idx, height)) = cache.get(&key) {
            let repeats = (1000000000000 - idx) / (i - idx) - 1;
            i += (i - idx) * repeats;
            total_height += (get_height(&chamber_map) - height) * repeats;
        } else {
            cache.insert(key, (i, get_height(&chamber_map)));
        }
        i += 1;
    }
}

fn print_chamber_map(chamber_map: [[u8; 7]; 10000]) {
    for h in chamber_map.iter().rev() {
        if h == &[0u8; 7] {
            continue;
        }
        for ch in h {
            if *ch == 0 {
                print!(".")
            } else {
                print!("{}", *ch as char);
            }
        }
        println!();
    }
}
