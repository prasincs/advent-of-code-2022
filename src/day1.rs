use crate::lines_from_file;

pub fn first_half() {
    let lines = lines_from_file("./inputs/day-1");
    let mut max_sum = 0;
    let mut curr_sum = 0;
    for line in lines {
        if line.is_empty() {
            if curr_sum > max_sum {
                max_sum = curr_sum;
            }
            curr_sum = 0;
            continue;
        }
        let parsed = line.parse::<i64>().unwrap();
        curr_sum += parsed;
    }
    if curr_sum > max_sum {
        max_sum = curr_sum;
    }
    println!("{}", max_sum);
}

pub fn second_half() {
    let lines = lines_from_file("./inputs/day-1");
    let mut sums: Vec<i64> = vec![];
    let mut curr_sum: i64 = 0;
    for line in lines {
        if line.is_empty() {
            sums.push(curr_sum);
            curr_sum = 0;
            continue;
        }
        let parsed = line.parse::<i64>().unwrap();
        curr_sum += parsed;
    }
    sums.push(curr_sum);
    // sort in reverse
    sums.sort_by(|a, b| b.cmp(a));
    println!("{}", sums.iter().take(3).sum::<i64>());
}

pub fn run() {
    first_half();
    second_half();
}
