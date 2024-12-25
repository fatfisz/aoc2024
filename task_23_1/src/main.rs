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

    let mut visited = HashSet::new();

    for (&first, first_set) in &edges {
        let first_vec = first_set.iter().cloned().collect::<Vec<_>>();
        for (second_index, &second) in first_vec.iter().enumerate() {
            let second_set = edges.get(second).unwrap();
            for &third in first_vec.iter().skip(second_index) {
                if second_set.contains(third)
                    && (first.starts_with('t') || second.starts_with('t') || third.starts_with('t'))
                {
                    let mut key = [first, second, third];
                    key.sort();
                    let key = key.join(",");
                    visited.insert(key);
                }
            }
        }
    }

    let result = visited.len();

    println!("{result}");
}
