use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;
use std::vec;

#[derive(Default, Debug)]
struct Monkey {
    Name: String,
    ID: u32,
    ItemWorries: Vec<u32>,
    Operation: (String, u32),
    TestDivisibleby: u32,
    TrueCase: u32,
    FalseCase: u32,
}

pub fn run() {
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-11-sample");
    let mut monkey: Monkey = Default::default();
    let mut monkeys: HashMap<u32, Monkey> = HashMap::new();
    for (line_num, line) in lines.iter().enumerate() {
        if line.ends_with(":") {
            println!("name {}", line);
            monkey.Name = line[..line.len() - 1].to_string();
            monkey.ID = line[..line.len() - 1]
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
        }
        if line.starts_with(" ") {
            let items: Vec<&str> = line.trim().split(":").collect();
            println!("{:?}", items);
            match items[0] {
                "Starting items" => {
                    let starting_items: Vec<u32> = items[1]
                        .strip_prefix(" ")
                        .unwrap()
                        .split(",")
                        .map(|x| x.trim().parse::<u32>().unwrap())
                        .collect();
                    monkey.ItemWorries = starting_items;
                }
                "Operation" => {
                    let op_items: Vec<&str> = items[1].split_ascii_whitespace().collect();
                    let mut op_symbol = op_items[3].to_string();
                    let mut op_num_str = op_items[4].trim();
                    let mut op_num = 0u32;
                    if op_num_str == "old" {
                        op_symbol = "pow".to_string();
                        op_num = 2;
                    } else {
                        op_num = op_num_str.parse::<u32>().unwrap();
                    }

                    monkey.Operation = (op_symbol, op_num)
                }
                "Test" => {
                    let divisibility = items[1]
                        .split_ascii_whitespace()
                        .last()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    monkey.TestDivisibleby = divisibility;
                }
                "If true" => {
                    let throw_to = items[1]
                        .split_ascii_whitespace()
                        .last()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    monkey.TrueCase = throw_to;
                }
                "If false" => {
                    let throw_to = items[1]
                        .split_ascii_whitespace()
                        .last()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    monkey.FalseCase = throw_to;
                }
                &_ => panic!("unsupported {}", items[0]),
            }
            // match items
            println!("{:?}", items);
        }
        if line == "" || line_num == lines.len() - 1 {
            monkeys.insert(monkey.ID, monkey);
            monkey = Default::default();
        }
    }
    let mut moves_map: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for (id, monkey) in monkeys.iter() {
        let mut moves: Vec<(u32, u32)> = vec![];
        println!("{:?}", monkey);
        for worry_item in monkey.ItemWorries.iter() {
            let new_worry_value = match monkey.Operation.0.as_str() {
                "*" => monkey.Operation.1 * worry_item,
                "+" => monkey.Operation.1 + worry_item,
                "pow" => worry_item * worry_item,
                &_ => {
                    println!("{:?}", monkey.Operation);
                    *worry_item
                }
            };
            let remainder = new_worry_value % monkey.TestDivisibleby;
            println!("worry={}, divisible by={}, remainder={}", new_worry_value, monkey.TestDivisibleby, remainder);
            if remainder == 0 {
                println!("move {} to monkey {}", worry_item, monkey.TrueCase);
                moves.push((*worry_item, monkey.TrueCase))
            } else {
                println!("move {} to monkey {}", worry_item, monkey.FalseCase);
                moves.push((*worry_item, monkey.FalseCase))
            }
        }
        moves_map.insert(*id, moves);
    }
    println!("{:?}", moves_map);
    for (id, moves) in moves_map {
        if let Some(monkey) = monkeys.get_mut(&id) {
            let to_remove: Vec<u32> = moves.iter().map(|x| x.0).collect();
            let mut current_worries = monkey.ItemWorries.clone();
            println!("[{}]{:?}, to_remove: {:?}", id, current_worries, to_remove);
            current_worries.retain(|x| !to_remove.contains(x));
            println!("[{}]{:?}", id, current_worries);
        };
        
    }
}
