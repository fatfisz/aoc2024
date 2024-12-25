use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim().split('\n').collect::<Vec<_>>();

    let mut gates = HashMap::new();
    let mut edges = HashMap::<&str, Vec<(&str, Op, &str)>>::new();

    let mut input_index = 0;
    loop {
        if input[input_index].len() == 0 {
            input_index += 1;
            break;
        }
        let line = input[input_index];
        let items = line.split(": ").collect::<Vec<_>>();
        let gate = items[0];
        let value = items[1] == "1";
        gates.insert(gate, value);

        input_index += 1;
    }
    while input_index < input.len() {
        let line = input[input_index];
        let items = line.split(' ').collect::<Vec<_>>();

        let gate1 = items[0];
        let op = match items[1] {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("Invalid operation"),
        };
        let gate2 = items[2];
        let target = items[4];

        edges
            .entry(gate1)
            .or_insert(vec![])
            .push((gate2, op, target));
        edges
            .entry(gate2)
            .or_insert(vec![])
            .push((gate1, op, target));

        input_index += 1;
    }

    let mut queue = gates.keys().cloned().collect::<Vec<_>>();
    let mut queue_index = 0;

    while queue_index < queue.len() {
        let gate1 = queue[queue_index];
        for (gate2, op, target) in edges.get(gate1).unwrap_or(&vec![]) {
            if gates.contains_key(gate2) {
                let first = *gates.get(gate1).unwrap();
                let second = *gates.get(gate2).unwrap();
                gates.insert(
                    target,
                    match op {
                        Op::And => first && second,
                        Op::Or => first || second,
                        Op::Xor => (first || second) && !(first && second),
                    },
                );
                queue.push(target);
            }
        }
        queue_index += 1;
    }

    let mut result: u64 = 0;

    for index in 0.. {
        let Some(&value) = gates.get(format!("z{:02}", index).as_str()) else {
            break;
        };

        if value {
            result |= 1 << index;
        }
    }

    println!("{result}");
}

#[derive(Clone, Copy, Debug)]
enum Op {
    And,
    Or,
    Xor,
}
