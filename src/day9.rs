use std::collections::HashMap;

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

pub fn run() {
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-9");
    let cmds = lines
        .iter()
        .map(|line| {
            let w: Vec<&str> = line.split_whitespace().collect();
            (w[0], w[1].parse::<u32>().unwrap())
        })
        .collect();
    part_one(cmds);
    // sample_test();
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

    part_one(cmds)
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
