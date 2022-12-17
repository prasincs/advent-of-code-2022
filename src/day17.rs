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

pub fn run() {
    // let input = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;
    let input = fs::read_to_string("./inputs/day-17").expect("Unable to read file");
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
            vec![' ', ' ', '#'],
            vec![' ', ' ', '#'],
            vec!['#', '#', '#'],
        ]),
        Symbol::from(vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']]),
        Symbol::from(vec![vec!['#', '#'], vec!['#', '#']]),
    ];

    // let symbols: [Symbol; 5] = [
    //     Symbol::from_rock_map(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
    //     Symbol::from_rock_map(vec![(0, 1), (1, 0), (1, 2), (2, 1)]),
    //     Symbol::from_rock_map(vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]),
    //     Symbol::from_rock_map(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
    //     Symbol::from_rock_map(vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
    // ];

    for symbol in &symbols {
        println!("symbol.rock_map = {:?}", symbol.rock_map);
    }

    // allocate length of the current
    // make tall enough chamber to fit full run
    let mut chamber_map = [[0u8; 7]; 10000];

    let (mut i, mut t, mut total_height) = (0, 0, 0);
    let mut cache = HashMap::new();
    while i < 1000000000000 {
        let symbol = &symbols[i % symbols.len()];
        let (mut h, mut w) = (get_height(&chamber_map) + 3, 2);
        loop {
            // println!("h = {:?}", h);
            // println!("w = {:?}", w);
            match input.as_bytes()[t % input.len()] {
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
                _ => {}
            };
            t += 1;
            if h == 0 || !symbol.fits_in_chamber(&chamber_map, h - 1, w) {
                break;
            }
            h -= 1;
            // h = h.wrapping_sub(1)
        }
        // println!("{:?}", symbol.rock_map.clone());
        for (dh, dw) in symbol.rock_map.clone() {
            // println!("dh, dw = {:?}, {:?}", dh, dw);
            // println!("h, w = {:?}, {:?}", h + dh, w + dw);
            chamber_map[h + dh][w + dw] = b'#';
        }

        let key = (
            i % symbols.len(),
            t % input.len(),
            chamber_heights(&chamber_map),
        );
        if let Some((idx, height)) = cache.get(&key) {
            let repeats = (1000000000000 - idx) / (i - idx) - 1;
            i += (i - idx) * repeats;
            total_height += (get_height(&chamber_map) - height) * repeats;
        } else {
            // println!("key={:?}, i={:?}", key, i);
            cache.insert(key, (i, get_height(&chamber_map)));
        }
        i += 1;
    }
    // cribbed from https://github.com/AxlLind/AdventOfCode2022/blob/main/src/bin/17.rs
    // println!("cache values={}", cache.values().len());
    let p1 = *cache
        .values()
        .find(|&&(i, _)| i == 2021)
        .map(|(_, h)| h)
        .unwrap();
    // 3230 was too large
    println!("{:?}", (p1, total_height + get_height(&chamber_map)));
}
