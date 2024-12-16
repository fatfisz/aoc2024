use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut result = 0;

    let mut line_iter = input.split('\n');
    let mut forbidden = HashSet::new();

    for line in &mut line_iter {
        if line.len() == 0 {
            break;
        }
        let numbers: Vec<_> = line.split('|').map(|s| s.parse::<i32>().unwrap()).collect();
        forbidden.insert((numbers[1], numbers[0]));
    }

    for line in &mut line_iter {
        let numbers: Vec<_> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        let is_valid = numbers.iter().enumerate().all(|(index, &first)| {
            numbers[index..]
                .iter()
                .all(|&second| !forbidden.contains(&(first, second)))
        });

        if is_valid {
            result += numbers[numbers.len() / 2];
        }
    }

    println!("{result}");
}
