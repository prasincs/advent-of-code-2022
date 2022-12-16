use petgraph::algo::floyd_warshall;
use petgraph::graph::Node;
use petgraph::{prelude::*, Directed, Graph};
use regex::Regex;
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref RE: regex::Regex =
        Regex::new(r"Valve (\S+) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.+)")
            .unwrap();
}

#[derive(Debug, Default, Clone)]
struct ValveInfo {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

pub fn run() {
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
    let mut graph: Graph<(), (), Directed> = Graph::new();
    let mut edges: Vec<(NodeIndex, NodeIndex)> = vec![];
    let mut idx_map: HashMap<String, NodeIndex> = HashMap::new();
    for valve_info in &valve_infos {
        println!("{:?}", valve_info);
        let idx = graph.add_node(());
        idx_map.insert(valve_info.name.clone(), idx);
    }

    for valve_info in &valve_infos {
        for lead in valve_info.tunnels.clone() {
            let connecting_idx = *idx_map.get(&lead).unwrap();
            edges.push((
                *idx_map.get(&valve_info.name.to_string()).unwrap(),
                connecting_idx,
            ));
        }
    }
    graph.extend_with_edges(edges.clone());
    let inf = std::i32::MAX;

    let res = floyd_warshall(&graph, |edge| -> i32 {
        if edges.contains(&(edge.source(), edge.target())) {
            1
        } else {
            inf
        }
    })
    .unwrap();

    for x in res {
        println!("{:?}", x);
    }
}

fn parse_valve_infos(input: &str) -> Vec<ValveInfo> {
    let caps: Vec<ValveInfo> = input
        .split("\n")
        .map(|line| {
            let caps = RE.captures(line).unwrap();
            let leading_to_valves: Vec<String> =
                caps[3].split(",").map(|x| x.trim().to_string()).collect();
            let name = caps[1].to_string();
            let flow_rate: u32 = caps[2].parse().unwrap();
            ValveInfo {
                name,
                flow_rate,
                tunnels: leading_to_valves,
            }
        })
        .collect();
    caps
}
