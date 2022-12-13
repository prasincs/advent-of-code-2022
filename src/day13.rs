use itertools::Itertools;
use serde::Deserialize;
use std::{cmp::Ordering, fs};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(untagged)]
enum Packet {
    List(Vec<Packet>),
    Int(i32),
}

/* we just need deserialize here and untagged allows us to not make keys AFAICT */

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            /* 4 cases
            (int, int)
            ([int], [int]),
            (int, [int]),
            ([int], int)
            */
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                // find the smollest
                for idx in 0..a.len().min(b.len()) {
                    let ord = a[idx].cmp(&b[idx]);
                    match ord {
                        Ordering::Equal => (),
                        _ => return ord,
                    }
                }
                // return the comparison, if everything so far matches because vector a needs to be smaller than vector b
                a.len().cmp(&b.len())
            }
            // encapsulate in list, so it now boils down to [int],[int]
            (Packet::Int(a), Packet::List(_b)) => Packet::List(vec![Packet::Int(*a)]).cmp(other),
            (Packet::List(_a), Packet::Int(b)) => self.cmp(&Packet::List(vec![Packet::Int(*b)])),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_twolists_simple_pair1() {
        assert_eq!(
            Ordering::Less,
            Packet::List(vec![
                Packet::Int(1),
                Packet::Int(1),
                Packet::Int(3),
                Packet::Int(1),
                Packet::Int(1)
            ])
            .cmp(&Packet::List(vec![
                Packet::Int(1),
                Packet::Int(1),
                Packet::Int(5),
                Packet::Int(1),
                Packet::Int(1)
            ]))
        )
    }

    #[test]
    fn test_list_of_list_and_int_pair2() {
        assert_eq!(
            Ordering::Less,
            Packet::List(vec![
                Packet::List(vec![Packet::Int(1)],),
                Packet::List(vec![Packet::Int(2), Packet::Int(3), Packet::Int(4)])
            ])
            .cmp(&Packet::List(vec![
                Packet::List(vec![Packet::Int(1)],),
                Packet::Int(4),
            ]))
        )
    }
    #[test]
    fn test_pair3() {
        assert_eq!(
            Ordering::Greater,
            Packet::List(vec![Packet::Int(9)]).cmp(&Packet::List(vec![
                Packet::Int(8),
                Packet::Int(7),
                Packet::Int(6),
            ]))
        )
    }

    #[test]
    fn test_pair7() {
        assert_eq!(
            Ordering::Greater,
            Packet::List(vec![Packet::List(vec![Packet::List(vec![])])])
                .cmp(&Packet::List(vec![Packet::List(vec![])]))
        )
    }
}

pub fn run() {
    let input = fs::read_to_string("./inputs/day-13").expect("Unable to read file");
    part_two_run(input.as_str())
}

fn sample_test() {
    let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    part_one_run(input);
}

fn part_one_run(input: &str) {
    let pairs = parse_packet_pairs(input);
    let mut sum = 0;
    for (idx, (a, b)) in pairs.iter().enumerate() {
        if let Ordering::Less = a.cmp(&b) {
            sum += idx + 1;
        }
    }
    println!("sum={}", sum);
}

fn part_two_run(input: &str) {
    let pairs = parse_packet_pairs(input);
    let dividers = [
        // [[2]]
        Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
    ];
    // https://stackoverflow.com/a/34745885
    let key = pairs
        .into_iter()
        .flat_map(|(x, y)| [x, y])
        .chain(dividers.clone()) // add in second iterator
        .sorted()
        .positions(|k| dividers.contains(&k))
        .fold(1, |sum, idx| sum * (idx + 1));
    println!("decoder key = {}", key);
}

fn parse_packet_pairs(input: &str) -> Vec<(Packet, Packet)> {
    let pairs = input
        .split("\n\n")
        .map(|lines| {
            lines
                .split("\n")
                .map(|line| parse_packet(line))
                .collect_tuple::<(_, _)>()
        })
        .collect::<Option<Vec<(Packet, Packet)>>>()
        .ok_or("invalid input string");
    pairs.unwrap()
}

fn parse_packet(line: &str) -> Packet {
    serde_json::from_str(line).unwrap()
}
