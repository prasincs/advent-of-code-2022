use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, Clone, Copy)]
struct MonkeyOp {
    id: u64,
    operator: Option<char>,
    value: i64,
    processed: bool,
    monkeys: [u64; 2],
}

impl MonkeyOp {
    pub fn id_from_string(name: &str) -> u64 {
        name.chars()
            .enumerate()
            .map(|(idx, c)| (c as u8 - b'a') as u64 * 26_u64.pow(idx as u32))
            .fold(0_u64, |a, x| a + x as u64)
    }

    pub fn string_from_id(idx: u64) -> String {
        let mut cur_idx = idx;
        let first_char = cur_idx % 26;
        cur_idx = cur_idx / 26;
        let second_char = cur_idx % 26;
        cur_idx = cur_idx / 26;
        let third_char = cur_idx % 26;
        cur_idx = cur_idx / 26;
        let fourth_char = cur_idx % 26;
        format!(
            "{}{}{}{}",
            ((first_char as u8 + b'a') as char),
            ((second_char as u8 + b'a') as char),
            ((third_char as u8 + b'a') as char),
            ((fourth_char as u8 + b'a') as char)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::MonkeyOp;

    #[test]
    fn test_char_conv() {
        assert_eq!(0, MonkeyOp::id_from_string("aaaa"));
        assert_eq!("aaaa", MonkeyOp::string_from_id(0));
        assert_eq!(343789, MonkeyOp::id_from_string("root"));
        assert_eq!("root", MonkeyOp::string_from_id(343789));
    }
}

pub fn run_sample() {
    let input = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;
    part_one_solve(input);
}

pub fn run() {
    let input = fs::read_to_string("./inputs/day-21").expect("Unable to read file");
    part_one_solve(input.as_str())
}

fn part_two_solve(input: &str) {
    let operations: Vec<(u64, MonkeyOp)> = input
        .split("\n")
        .map(|line| -> (u64, MonkeyOp) {
            let elems: Vec<&str> = line.split(":").collect();
            let id_str = String::from(elems[0]);
            let mut value = 0;
            let mut operator: Option<char> = None;
            let mut monkeys: [u64; 2] = [0_u64; 2];
            let rhs = elems[1];
            let test = rhs.trim().parse::<i32>();
            match test {
                Ok(num) => value = num,
                Err(e) => {
                    let items: Vec<&str> = rhs.split_ascii_whitespace().collect();
                    println!("[{}] items = {:?}", id_str, items);
                    monkeys[0] = MonkeyOp::id_from_string(items[0]);
                    monkeys[1] = MonkeyOp::id_from_string(items[2]);
                    operator = Some(items[1].chars().next().unwrap())
                }
            }
            let idx = MonkeyOp::id_from_string(&id_str);
            (
                idx,
                MonkeyOp {
                    id: idx,
                    operator,
                    processed: false,
                    value: value.into(),
                    monkeys,
                },
            )
        })
        .collect::<Vec<(u64, MonkeyOp)>>();
    // filter out monkeys with humn as input
    let filtered_operations = operations
        .iter()
        .filter(|(id, _)| id == &MonkeyOp::id_from_string("humn"));
    let mut monkey_map: HashMap<u64, MonkeyOp> = operations.clone().into_iter().collect();
    let mut sums: HashMap<u64, i64> = HashMap::new();
    let mut queue: VecDeque<MonkeyOp> = VecDeque::new();
    for (id, op) in &operations.clone() {
        match op.operator {
            None => {
                println!("processing {}", id);
                monkey_map.entry(*id).and_modify(|k| k.processed = true);
                sums.entry(*id).or_insert(op.value);
            }
            Some(_) => {
                // if some operator, push those monkeys into queue
                for monkey in op.monkeys {
                    let child_op = *monkey_map.get(&monkey).unwrap();
                    queue.push_back(child_op.clone());
                }
                queue.push_back(op.clone());
            }
        }
    }
    for (k, v) in &monkey_map {
        println!("k={:?}, v={:?}", k, v);
    }
    while queue.len() > 0 {
        let mut monkey = queue.pop_front().unwrap();
        if monkey.monkeys == [0, 0] {
            continue;
        }
        println!(
            "processing = {:?}, monkeys = {:?}",
            monkey.id,
            monkey.monkeys.map(|f| MonkeyOp::string_from_id(f))
        );
        let mut val = 0;
        let mut children_processed = monkey
            .monkeys
            .iter()
            .all(|f| monkey_map.get(&f).unwrap().processed);
        for child in monkey.monkeys {
            if child == 0 {
                continue;
            }
            let childop = *monkey_map.get(&child).unwrap();
            if !childop.processed {
                queue.push_back(childop);
            }
        }

        if !children_processed {
            queue.push_back(monkey);
        } else {
            println!("children processed");
            let monkey_0 = *sums.get(&monkey.monkeys[0]).unwrap();
            let monkey_1 = *sums.get(&monkey.monkeys[1]).unwrap();
            match monkey.operator {
                Some('-') => val = monkey_0 - monkey_1,
                Some('+') => val = monkey_0 + monkey_1,
                Some('*') => val = monkey_0 * monkey_1,
                Some('/') => val = monkey_0 / monkey_1,
                None | _ => panic!("improbable"),
            }
            sums.insert(monkey.id, val);
            monkey.value = val;
            monkey.processed = true;
            monkey_map
                .entry(monkey.id)
                .and_modify(|k| k.processed = true);
        }
    }
    for (id, sum) in &sums {
        println!(
            "id {:?}, name={:?},sum = {:?}",
            id,
            MonkeyOp::string_from_id(*id),
            sum
        );
    }
    println!("root = {:?}", &sums.get(&MonkeyOp::id_from_string("root")));
}

fn part_one_solve(input: &str) {
    let operations: Vec<(u64, MonkeyOp)> = input
        .split("\n")
        .map(|line| -> (u64, MonkeyOp) {
            let elems: Vec<&str> = line.split(":").collect();
            let id_str = String::from(elems[0]);
            let mut value = 0;
            let mut operator: Option<char> = None;
            let mut monkeys: [u64; 2] = [0_u64; 2];
            let rhs = elems[1];
            let test = rhs.trim().parse::<i32>();
            match test {
                Ok(num) => value = num,
                Err(e) => {
                    let items: Vec<&str> = rhs.split_ascii_whitespace().collect();
                    println!("[{}] items = {:?}", id_str, items);
                    monkeys[0] = MonkeyOp::id_from_string(items[0]);
                    monkeys[1] = MonkeyOp::id_from_string(items[2]);
                    operator = Some(items[1].chars().next().unwrap())
                }
            }
            let idx = MonkeyOp::id_from_string(&id_str);
            (
                idx,
                MonkeyOp {
                    id: idx,
                    operator,
                    processed: false,
                    value: value.into(),
                    monkeys,
                },
            )
        })
        .collect::<Vec<(u64, MonkeyOp)>>();
    let mut monkey_map: HashMap<u64, MonkeyOp> = operations.clone().into_iter().collect();
    let mut sums: HashMap<u64, i64> = HashMap::new();
    let mut queue: VecDeque<MonkeyOp> = VecDeque::new();
    for (id, op) in &operations.clone() {
        match op.operator {
            None => {
                println!("processing {}", id);
                monkey_map.entry(*id).and_modify(|k| k.processed = true);
                sums.entry(*id).or_insert(op.value);
            }
            Some(_) => {
                // if some operator, push those monkeys into queue
                for monkey in op.monkeys {
                    let child_op = *monkey_map.get(&monkey).unwrap();
                    queue.push_back(child_op.clone());
                }
                queue.push_back(op.clone());
            }
        }
    }
    for (k, v) in &monkey_map {
        println!("k={:?}, v={:?}", k, v);
    }
    while queue.len() > 0 {
        let mut monkey = queue.pop_front().unwrap();
        if monkey.monkeys == [0, 0] {
            continue;
        }
        println!(
            "processing = {:?}, monkeys = {:?}",
            monkey.id,
            monkey.monkeys.map(|f| MonkeyOp::string_from_id(f))
        );
        let mut val = 0;
        let mut children_processed = monkey
            .monkeys
            .iter()
            .all(|f| monkey_map.get(&f).unwrap().processed);
        for child in monkey.monkeys {
            if child == 0 {
                continue;
            }
            let childop = *monkey_map.get(&child).unwrap();
            if !childop.processed {
                queue.push_back(childop);
            }
        }

        if !children_processed {
            queue.push_back(monkey);
        } else {
            println!("children processed");
            let monkey_0 = *sums.get(&monkey.monkeys[0]).unwrap();
            let monkey_1 = *sums.get(&monkey.monkeys[1]).unwrap();
            match monkey.operator {
                Some('-') => val = monkey_0 - monkey_1,
                Some('+') => val = monkey_0 + monkey_1,
                Some('*') => val = monkey_0 * monkey_1,
                Some('/') => val = monkey_0 / monkey_1,
                None | _ => panic!("improbable"),
            }
            sums.insert(monkey.id, val);
            monkey.value = val;
            monkey.processed = true;
            monkey_map
                .entry(monkey.id)
                .and_modify(|k| k.processed = true);
        }
    }
    for (id, sum) in &sums {
        println!(
            "id {:?}, name={:?},sum = {:?}",
            id,
            MonkeyOp::string_from_id(*id),
            sum
        );
    }
    println!("root = {:?}", &sums.get(&MonkeyOp::id_from_string("root")));
}
