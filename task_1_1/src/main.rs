use std::{fs::read_to_string, iter::zip};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut first_list = Vec::<i32>::new();
    let mut second_list = Vec::<i32>::new();

    for line in input.split('\n') {
        let mut numbers = line.split_whitespace();
        first_list.push(numbers.next().unwrap().parse().unwrap());
        second_list.push(numbers.next().unwrap().parse().unwrap());
    }

    first_list.sort();
    second_list.sort();

    let mut distance_sum = 0;

    for (&first, &second) in zip(&first_list, &second_list) {
        distance_sum += first.abs_diff(second);
    }

    println!("{distance_sum}");
}
