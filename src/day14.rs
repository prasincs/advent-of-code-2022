use itertools::Itertools;

pub fn run() {
    let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
    //  Vec<Vec<(i32, i32)>>
    // get something like [[(498, 4), (498, 6), (496, 6)], [(503, 4), (502, 4), (502, 9), (494, 9)]]
    let rocks = input
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
    println!("{:?}", rocks);
}
