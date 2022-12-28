use std::fs;

use itertools::Itertools;

fn convert_snafu_to_int(snafu: &str) -> i64 {
    let mut result = 0_i64;
    for (idx, n) in snafu.chars().enumerate() {
        let pos = snafu.len() - idx - 1;
        // println!("idx = {:?}, n={:?}", idx, n);
        match n {
            '-' => {
                let k = 1_i64 * (5_i64.pow(pos as u32)) as i64;
                // println!("k = {:?}", k);
                result -= k;
            }
            '=' => {
                let k = 2_i64 * (5_i64.pow(pos as u32)) as i64;
                // println!("k = {:?}", k);
                result -= k;
            }
            _ => {
                let k = n.to_digit(10).unwrap() as i64 * (5_i64.pow(pos as u32)) as i64;
                // println!("k = {:?}", k);
                result += k;
            }
        }
    }
    return result;
}

fn convert_int_to_snafu(num: i64) -> String {
    let mut input = num;
    let mut values = Vec::new();
    while input > 0 {
        let mut div = input / 5;
        let mut rem = input % 5;
        if rem > 2 {
            div = div + 1;
            rem = rem - 5_i64;
        }
        values.push(rem);
        input = div;
    }

    let mut result: String = Default::default();
    for value in values.iter().rev() {
        let c: char = match value {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        };
        result.push_str(format!("{}", c).as_str());
    }
    return result;
}

#[cfg(test)]
mod test_snafu {
    use super::*;

    #[test]
    fn test_snafu() {
        // assert_eq!(1, convert_snafu_to_int("1"));
        // assert_eq!(2, convert_snafu_to_int("2"));
        assert_eq!(3, convert_snafu_to_int("1="));
        assert_eq!(4, convert_snafu_to_int("1-"));
        assert_eq!(5, convert_snafu_to_int("10"));
        assert_eq!(6, convert_snafu_to_int("11"));
        assert_eq!(7, convert_snafu_to_int("12"));
        assert_eq!(8, convert_snafu_to_int("2="));
        assert_eq!(9, convert_snafu_to_int("2-"));
        assert_eq!(15, convert_snafu_to_int("1=0"));
        assert_eq!(12345, convert_snafu_to_int("1-0---0"));
        assert_eq!(314159265, convert_snafu_to_int("1121-1110-1=0"));
    }

    #[test]
    fn test_int_to_snafu() {
        assert_eq!("1=", convert_int_to_snafu(3));
        assert_eq!("10", convert_int_to_snafu(5));
        assert_eq!("11", convert_int_to_snafu(6));
        assert_eq!("12", convert_int_to_snafu(7));
        assert_eq!("2=", convert_int_to_snafu(8));
        assert_eq!("1121-1110-1=0", convert_int_to_snafu(314159265));
    }
}

fn part_one() {
    let input = fs::read_to_string("./inputs/day-25").expect("Unable to read file");
    let sum: i64 = input.split("\n").map(convert_snafu_to_int).sum();
    println!("sum as snafu= {:?}", convert_int_to_snafu(sum));
}

pub fn run() {
    part_one()
}

fn run_sample() {
    let input = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;

    let sum: i64 = input.split("\n").map(convert_snafu_to_int).sum();
    println!("sum = {:?}", sum);
    println!("sum as snafu= {:?}", convert_int_to_snafu(sum));
}
