use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;
use std::vec;

#[derive(Default, Clone, Debug, PartialEq,PartialOrd, Eq)]
struct Monkey {
    Name: String,
    ID: u32,
    ItemWorries: Vec<u32>,
    Operation: (String, u32),
    TestDivisibleby: u32,
    TrueCase: u32,
    FalseCase: u32,
}


impl Ord for Monkey{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.ID).cmp(&other.ID)
    }
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

    run_round(0, &mut monkeys);
    run_round(1, &mut monkeys);
    // println!("{:?}", moves_map);
    // for (id, moves) in moves_map {
    //     if let Some(monkey) = monkeys.get_mut(&id) {
    //         let to_remove: Vec<u32> = moves.iter().map(|x| x.0).collect();
    //         let mut current_worries = monkey.ItemWorries.clone();
    //         println!("[{}]{:?}, to_remove: {:?}", id, current_worries, to_remove);
    //         current_worries.retain(|x| !to_remove.contains(x));
    //         println!("[{}]{:?}", id, current_worries);
    //     };

    // }
}

fn run_round(round: u32, monkeys: &mut HashMap<u32, Monkey>) {
    let mut new_worry_items: Vec<(u32,u32)> = vec![];
    // we are going to mutate the original monkeys, so use a clone
    for (id, monkey) in monkeys.clone().iter().sorted() {
        let mut cur_item_worries = monkey.ItemWorries.clone();
        cur_item_worries.extend(new_worry_items.iter().filter(|(move_id,_)| move_id == id ).map(|(_, worry_id)| worry_id));
        for worry_item in cur_item_worries{
            let current_worry_value = match monkey.Operation.0.as_str() {
                "*" => monkey.Operation.1 * worry_item,
                "+" => monkey.Operation.1 + worry_item,
                "pow" => worry_item * worry_item,
                &_ => {
                    // println!("{:?}", monkey.Operation);
                    panic!("Invalid operation {:?}", monkey.Operation);
                }
            };
            let new_worry_value: u32 = current_worry_value / 3; 
            let remainder = new_worry_value % monkey.TestDivisibleby;
            println!(
                "monkey={},worry={}, divisible by={}, new_worry={}, remainder={}",
                monkey.ID,
                current_worry_value, monkey.TestDivisibleby, new_worry_value, remainder
            );
            if remainder == 0 {
                println!("move {} to monkey {}", worry_item, monkey.TrueCase);
                let moved_monkey = monkeys.get_mut(&monkey.TrueCase).unwrap();
                moved_monkey.ItemWorries.push(new_worry_value);
                new_worry_items.push((monkey.TrueCase, new_worry_value));
            } else {
                println!("move {} to monkey {}", worry_item, monkey.FalseCase);
                let moved_monkey = monkeys.get_mut(&monkey.FalseCase).unwrap();
                moved_monkey.ItemWorries.push(new_worry_value);
                new_worry_items.push((monkey.FalseCase, new_worry_value))
            }
            let cur_monkey = monkeys.get_mut(&monkey.ID).unwrap();
            cur_monkey.ItemWorries.retain(|&x| x != worry_item);
        }
    }
    println!("Round {}", round);
    for monkey in monkeys{
        println!("{:?}", monkey);
    }
}
