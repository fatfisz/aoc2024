use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut number_list = Vec::<i32>::new();
    let mut counts = HashMap::<i32, i32>::new();

    for line in input.split('\n') {
        let mut numbers = line.split_whitespace();
        number_list.push(numbers.next().unwrap().parse().unwrap());
        let second = numbers.next().unwrap().parse().unwrap();
        counts.insert(second, counts.get(&second).unwrap_or(&0) + 1);
    }

    let mut similarity_score = 0;

    for number in number_list {
        similarity_score += counts.get(&number).unwrap_or(&0) * number;
    }

    println!("{similarity_score}");
}
