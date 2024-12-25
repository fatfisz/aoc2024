use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim().split('\n');

    let mut edges = HashMap::<&str, HashSet<&str>>::new();

    for edge in input {
        let mut iter = edge.split('-');
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        edges
            .entry(first)
            .and_modify(|set| {
                set.insert(second);
            })
            .or_insert_with(|| HashSet::from([second]));
        edges
            .entry(second)
            .and_modify(|set| {
                set.insert(first);
            })
            .or_insert_with(|| HashSet::from([first]));
    }

    let mut sets = HashSet::new();
    for (&first, first_set) in &edges {
        for &second in first_set {
            let mut key = vec![first, second];
            key.sort();
            sets.insert(key);
        }
    }

    loop {
        let mut bigger_sets = HashSet::new();

        for set in sets.iter() {
            let head = set[0];
            let tail = &set[1..];

            for &head_edge in edges.get(head).unwrap() {
                if !tail.contains(&head_edge)
                    && tail
                        .iter()
                        .all(|tail_vertex| edges.get(tail_vertex).unwrap().contains(head_edge))
                {
                    let mut new_set = set.clone();
                    new_set.push(head_edge);
                    new_set.sort();
                    bigger_sets.insert(new_set);
                }
            }
        }

        if bigger_sets.len() == 0 {
            break;
        }
        sets = bigger_sets;
    }

    let result = sets.iter().next().unwrap().join(",");

    println!("{result}");
}
