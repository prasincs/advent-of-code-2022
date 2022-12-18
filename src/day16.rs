use itertools::Itertools;
use regex::Regex;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::{fs, vec};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref RE: regex::Regex =
        Regex::new(r"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
}

#[derive(Debug, Default, Clone)]
struct ValveInfo {
    index: usize,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl ValveInfo {
    fn index_from(s: &str) -> usize {
        let chars = s.as_bytes();
        (chars[0] - b'A') as usize * 26 + (chars[1] - b'A') as usize
    }
    fn index_to_str(idx: usize) -> String {
        let first_char = idx / 26;
        let second_char = idx % 26;
        format!(
            "{}{}",
            ((first_char as u8 + b'A') as char),
            ((second_char as u8 + b'A') as char)
        )
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_char_conv() {
        let dd_idx = ValveInfo::index_from("DD");
        assert_eq!(String::from("AA"), ValveInfo::index_to_str(0));
        assert_eq!(String::from("DD"), ValveInfo::index_to_str(dd_idx));
    }
}

fn part_one() {
    let input = String::from(
        r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#,
    );
    // let input = fs::read_to_string("./inputs/day-16").expect("Unable to read file");
    solve(input.as_str(), "AA", 30);
}

fn part_two() {
    let input = fs::read_to_string("./inputs/day-16").expect("Unable to read file");
    // solve(input.as_str(), "AA", 30);
    // let (valves, pressure) = solve(input.as_str(), "AA", 26);
    // println!("valves len={}", valves.len());

    let valve_infos = parse_valve_infos(input.as_str());
    //floyd_warshall_library(valve_infos);
    let distance_grid: Vec<Vec<u32>> = create_distance_grid(&valve_infos);
    // pick valves with flow rate > 0 to start with
    let closed_valves: Vec<&ValveInfo> = valve_infos
        .iter()
        .filter(|valve| valve.flow_rate > 0)
        .collect();
    // let start = ValveInfo::index_from("AA");
    let mut pressure_map: HashMap<(usize, usize), u32> = HashMap::new();
    let valve_indices: Vec<usize> = valve_infos.iter().map(|x| x.index).collect();
    for start in valve_indices.iter().permutations(2) {
        println!(
            "processing with starts={:?}",
            start.iter().map(|x| ValveInfo::index_to_str(**x)).join(",")
        );
        let (total_pressure0, remaining_idx) =
            best_path(&distance_grid, closed_valves.clone(), *start[0], 26);
        let remaining: Vec<&ValveInfo> = closed_valves
            .clone()
            .into_iter()
            .filter(|k| !remaining_idx.contains(&k.index))
            .collect();
        println!(
            "remaining={:?}",
            remaining
                .iter()
                .map(|x| ValveInfo::index_to_str(x.index))
                .join(",")
        );
        let (total_pressure1, _) = best_path(&distance_grid, remaining.clone(), *start[1], 26);
        pressure_map.insert((*start[0], *start[1]), total_pressure0 + total_pressure1);
    }
    println!("max={}", pressure_map.values().max().unwrap());
}

pub fn run() {
    part_one();
    // run_sample();
    // part_two();
}

fn run_sample() {
    /*
    DD -> CC=2, AA=0, EE=3
    CC */
    // https://docs.rs/petgraph/latest/petgraph/algo/floyd_warshall/fn.floyd_warshall.html

    let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
    let valve_infos = parse_valve_infos(input);
    //floyd_warshall_library(valve_infos);
    let distance_grid: Vec<Vec<u32>> = create_distance_grid(&valve_infos);
    // pick valves with flow rate > 0 to start with
    let closed_valves: Vec<&ValveInfo> = valve_infos
        .iter()
        .filter(|valve| valve.flow_rate > 0)
        .collect();
    // let start = ValveInfo::index_from("AA");
    let mut pressure_map: HashMap<(usize, usize), u32> = HashMap::new();
    let valve_indices: Vec<usize> = valve_infos.iter().map(|x| x.index).collect();
    for start in valve_indices.iter().permutations(2) {
        println!(
            "processing with starts={:?}",
            start.iter().map(|x| ValveInfo::index_to_str(**x)).join(",")
        );
        let (total_pressure0, remaining_idx) =
            best_path(&distance_grid, closed_valves.clone(), *start[0], 26);
        let remaining: Vec<&ValveInfo> = closed_valves
            .clone()
            .into_iter()
            .filter(|k| !remaining_idx.contains(&k.index))
            .collect();
        println!(
            "remaining={:?}",
            remaining
                .iter()
                .map(|x| ValveInfo::index_to_str(x.index))
                .join(",")
        );
        let (total_pressure1, _) = best_path(&distance_grid, remaining.clone(), *start[1], 26);
        pressure_map.insert((*start[0], *start[1]), total_pressure0 + total_pressure1);
    }
    println!("max={}", pressure_map.values().max().unwrap());
    // for (x,y) in pressure_map.keys(){}
    // println!("total_pressure={}", total_pressure);

    // let (valves, pressure) = solve(input, "AA", 26);
    // println!("valves len={}", valves.len());
}

fn solve(input: &str, start_valve: &str, minutes_left: u32) -> (Vec<ValveInfo>, u32) {
    let valve_infos = parse_valve_infos(input);
    //floyd_warshall_library(valve_infos);
    let distance_grid: Vec<Vec<u32>> = create_distance_grid(&valve_infos);
    // pick valves with flow rate > 0 to start with
    let closed_valves: Vec<&ValveInfo> = valve_infos
        .iter()
        .filter(|valve| valve.flow_rate > 0)
        .collect();
    let start = ValveInfo::index_from(start_valve);
    let (total_pressure, opened_valves) =
        best_path(&distance_grid, closed_valves, start, minutes_left);
    println!("total_pressure={}", total_pressure);
    println!(
        "opened valves={:?}",
        opened_valves
            .iter()
            .map(|x| ValveInfo::index_to_str(*x))
            .join(",")
    );
    (valve_infos, total_pressure)
}

//struct PressureAndOpenedList(u32, Vec<usize>);

fn best_path(
    distances: &Vec<Vec<u32>>,
    closed_valves: Vec<&ValveInfo>,
    at_valve: usize,
    minutes_left: u32,
) -> (u32, Vec<usize>) {
    let mut pressures: Vec<(u32, Vec<usize>)> = vec![];
    let mut opened_valves: HashSet<usize> = HashSet::new();
    for valve in &closed_valves {
        let distance = distances[at_valve][valve.index];

        if distance >= minutes_left {
            continue;
        }

        let pressure = valve.flow_rate * (minutes_left - distance - 1);

        let remaining: Vec<&ValveInfo> = closed_valves
            .iter()
            .filter(|v| v.index != valve.index)
            .cloned()
            .collect();
        let (new_pressure, new_opened_valves) = best_path(
            distances,
            remaining.clone(),
            valve.index,
            minutes_left - distance - 1,
        );
        // for r in remaining {
        if valve.flow_rate > 0 {
            opened_valves.insert(valve.index);
        }
        // }
        let best_pressure = pressure + new_pressure;
        for v in new_opened_valves {
            opened_valves.insert(v);
        }
        pressures.push((
            best_pressure,
            opened_valves.clone().into_iter().collect_vec(),
        ));
    }

    if let Some((x, y)) = pressures.into_iter().max_by_key(|x| x.0) {
        (x, y)
    } else {
        (0u32, vec![])
    }
    // .unwrap_or(&(0u32, vec![0usize; 0]))
}

fn create_distance_grid(valve_infos: &Vec<ValveInfo>) -> Vec<Vec<u32>> {
    let last_index = ValveInfo::index_from("ZZ");
    let mut edges: Vec<Vec<usize>> = vec![vec![]; last_index + 1];
    let vertices: Vec<usize> = valve_infos.iter().map(|valve| valve.index).collect();

    for valve_info in valve_infos {
        for tunnel in &valve_info.tunnels {
            let tun_str = tunnel.as_str();
            let tun_index = ValveInfo::index_from(tun_str);
            edges[valve_info.index].push(tun_index);
        }
    }
    let mut grid: Vec<Vec<u32>> = vec![vec![0; last_index + 1]; last_index + 1];

    for valve_info in valve_infos {
        let distance_vec = dijkstra(&edges, &vertices, last_index, valve_info.index);
        grid[valve_info.index] = distance_vec;
    }

    grid
}

fn dijkstra(edges: &[Vec<usize>], vertices: &[usize], last_index: usize, start: usize) -> Vec<u32> {
    let mut distance_to: Vec<u32> = vec![u32::MAX - 1; last_index + 1];
    let mut queue: HashSet<usize> = vertices.iter().cloned().collect();

    // distance to itself is 0
    distance_to[start] = 0;
    while !queue.is_empty() {
        let shortest_dist_idx = *queue
            .iter()
            .min_by(|&&a, &&b| distance_to[a].cmp(&distance_to[b]))
            .unwrap();

        queue.remove(&shortest_dist_idx);

        let neighbours: Vec<usize> = edges[shortest_dist_idx]
            .iter()
            .filter(|valve| queue.contains(valve))
            .cloned()
            .collect();

        for neighbor in neighbours {
            let alt = distance_to[shortest_dist_idx] + 1;

            if alt < distance_to[neighbor] {
                distance_to[neighbor] = alt;
            }
        }
    }
    distance_to
}

fn parse_valve_infos(input: &str) -> Vec<ValveInfo> {
    let caps: Vec<ValveInfo> = input
        .split("\n")
        .map(|line| {
            println!("{}", line);
            let caps = RE.captures(line).unwrap();
            let leading_to_valves: Vec<String> =
                caps[3].split(",").map(|x| x.trim().to_string()).collect();
            let flow_rate: u32 = caps[2].parse().unwrap();
            let index = ValveInfo::index_from(&caps[1]);
            ValveInfo {
                index,
                flow_rate,
                tunnels: leading_to_valves,
            }
        })
        .collect();
    caps
}
