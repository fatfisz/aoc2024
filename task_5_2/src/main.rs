use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut result = 0;

    let mut line_iter = input.split('\n');
    let mut forbidden = HashSet::<_>::new();

    for line in &mut line_iter {
        if line.len() == 0 {
            break;
        }
        let numbers: Vec<_> = line.split('|').map(|s| s.parse::<i32>().unwrap()).collect();
        forbidden.insert((numbers[1], numbers[0]));
    }

    for line in &mut line_iter {
        let numbers: Vec<_> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        let mut ordered_numbers = numbers.clone();
        ordered_numbers.sort_by(|&a, &b| {
            if forbidden.contains(&(a, b)) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        if numbers != ordered_numbers {
            result += ordered_numbers[ordered_numbers.len() / 2];
        }
    }

    println!("{result}");
}
