use std::{collections::HashMap, mem::replace, vec};

fn compute_position_tail(
    pos_h: (i32, i32),
    pos_t: (i32, i32),
    visited_tail: &mut HashMap<(i32, i32), bool>,
) -> (i32, i32) {
    let mut x = pos_t.0;
    let mut y = pos_t.1;
    let delta = (pos_h.0 - pos_t.0, pos_h.1 - pos_t.1);
    if delta.0.abs() == 2 || delta.1.abs() == 2 {
        x += delta.0.signum();
        y += delta.1.signum();
    }
    visited_tail.insert((x, y), true);
    (x, y)
}

fn compute_positions(
    pos_ropes: Vec<(i32, i32)>,
    visited_tail: &mut HashMap<(i32, i32), bool>,
) -> Vec<(i32, i32)> {
    let mut new_vec: Vec<(i32, i32)> = vec![];
    new_vec.push(pos_ropes[0]);
    for i in 1..pos_ropes.len() {
        let prev_elem_idx = i - 1;
        let pos_before = new_vec.get(prev_elem_idx).unwrap();
        let pos = pos_ropes.get(i).unwrap();
        let mut x: i32 = pos.0;
        let mut y: i32 = pos.1;
        let delta = (pos_before.0 - pos.0, pos_before.1 - pos.1);
        if delta.0.abs() == 2 || delta.1.abs() == 2 {
            x += delta.0.signum();
            y += delta.1.signum();
        }
        new_vec.push((x, y));
    }
    // println!("last: {:?}", new_vec.last());
    visited_tail.insert((new_vec.last().unwrap().0, new_vec.last().unwrap().1), true);
    new_vec
}

pub fn run() {
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-9");
    let cmds: Vec<(&str, u32)> = lines
        .iter()
        .map(|line| {
            let w: Vec<&str> = line.split_whitespace().collect();
            (w[0], w[1].parse::<u32>().unwrap())
        })
        .collect();
    // part_one(cmds);
    // bigger_sample_test();
    part_two(cmds);
}

fn bigger_sample_test() {
    let input: String = String::from(
        r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#,
    );
    let cmds: Vec<(&str, u32)> = input
        .split('\n')
        .map(|line| {
            let w: Vec<&str> = line.split_whitespace().collect();
            (w[0], w[1].parse::<u32>().unwrap())
        })
        .collect();

    // println!("{:?}",compute_position_tail((posH.0+1, posH.1), posT));
    // println!("{:?}",compute_position_tail((posH.0+2, posH.1), posT));
    // println!("{:?}",compute_position_tail((posH.0+3, posH.1), (posT.0+1, posT.1)));

    part_two(cmds)
}

fn sample_test() {
    let input: String = String::from(
        r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
    );
    let cmds: Vec<(&str, u32)> = input
        .split('\n')
        .map(|line| {
            let w: Vec<&str> = line.split_whitespace().collect();
            (w[0], w[1].parse::<u32>().unwrap())
        })
        .collect();

    // println!("{:?}",compute_position_tail((posH.0+1, posH.1), posT));
    // println!("{:?}",compute_position_tail((posH.0+2, posH.1), posT));
    // println!("{:?}",compute_position_tail((posH.0+3, posH.1), (posT.0+1, posT.1)));

    part_two(cmds)
}

fn part_two(cmds: Vec<(&str, u32)>) {
    let mut pos_ropes: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    // assert_eq!(10, ropes.len())
    // let _got = replace(&mut ropes[0], (2,0));
    // ropes = compute_positions(ropes);
    let mut visited_t: HashMap<(i32, i32), bool> = HashMap::new();
    visited_t.insert((0, 0), true);
    for (cmd, len) in cmds {
        println!("cmd={},len={}", cmd, len);
        println!("{:?}", pos_ropes);
        for row in -20..20 {
            for col in -20..20 {
                let mut to_print: String = ".".to_string();
                for (i, (x, y)) in pos_ropes.iter().enumerate() {
                    if (row, col) == (*x, *y) {
                        if i == 0 {
                            to_print = "H".to_string();
                        } else {
                            to_print = format!("{}", i);
                        }
                    }
                }
                print!("{}", to_print);
                // if printed > 0 {
                //     printed-=1;
                // }else {
                //     print!(".");
                // }
            }
            println!()
        }
        match cmd {
            "R" => {
                for _i in 0..len {
                    let pos_h = pos_ropes[0];
                    let _got = replace(&mut pos_ropes[0], (pos_h.0, pos_h.1 + 1));
                    pos_ropes = compute_positions(pos_ropes, &mut visited_t);
                }
            }
            "U" => {
                for _i in 0..len {
                    let pos_h = pos_ropes[0];
                    let _got = replace(&mut pos_ropes[0], (pos_h.0 + 1, pos_h.1));
                    pos_ropes = compute_positions(pos_ropes, &mut visited_t);
                }
            }
            "D" => {
                for _i in 0..len {
                    let pos_h = pos_ropes[0];
                    let _got = replace(&mut pos_ropes[0], (pos_h.0 - 1, pos_h.1));
                    pos_ropes = compute_positions(pos_ropes, &mut visited_t);
                }
            }
            "L" => {
                for _i in 0..len {
                    let pos_h = pos_ropes[0];
                    let _got = replace(&mut pos_ropes[0], (pos_h.0, pos_h.1 - 1));
                    pos_ropes = compute_positions(pos_ropes, &mut visited_t);
                }
            }
            &_ => todo!(),
        }
    }
    println!("{}", visited_t.len());
}

fn part_one(cmds: Vec<(&str, u32)>) {
    let mut visited_t: HashMap<(i32, i32), bool> = HashMap::new();
    visited_t.insert((0, 0), true);
    let mut pos_h = (0i32, 0i32);
    let mut pos_t = (0i32, 0i32);
    for (cmd, len) in cmds {
        println!("cmd={},len={}", cmd, len);
        match cmd {
            "R" => {
                for _i in 0..len {
                    pos_h = (pos_h.0, pos_h.1 + 1);
                    pos_t = compute_position_tail(pos_h, pos_t, &mut visited_t);
                    println!("R{:?},{:?}", pos_h, pos_t);
                }
            }
            "U" => {
                for _i in 0..len {
                    pos_h = (pos_h.0 + 1, pos_h.1);
                    pos_t = compute_position_tail(pos_h, pos_t, &mut visited_t);
                    println!("U{:?},{:?}", pos_h, pos_t);
                }
            }
            "D" => {
                for _i in 0..len {
                    pos_h = (pos_h.0 - 1, pos_h.1);
                    pos_t = compute_position_tail(pos_h, pos_t, &mut visited_t);
                    println!("D{:?},{:?}", pos_h, pos_t);
                }
            }
            "L" => {
                for _i in 0..len {
                    pos_h = (pos_h.0, pos_h.1 - 1);
                    pos_t = compute_position_tail(pos_h, pos_t, &mut visited_t);
                    println!("L{:?},{:?}", pos_h, pos_t);
                }
            }
            &_ => todo!(),
        }
    }
    println!("{}", visited_t.len());
    // for (x, y) in visited_t.keys() {
    //     println!("({},{})", x, y);
    // }
}
