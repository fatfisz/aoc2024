use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();
    let input = input.split('\n').collect::<Vec<_>>();

    let towels = input[0].split(", ").collect::<Vec<_>>();
    let mut result = 0;

    for line in &input[2..] {
        let mut possible = vec![0_usize; line.len() + 1];
        possible[0] = 1;

        for index in 0..line.len() {
            for towel in &towels {
                if line[index..].starts_with(towel) && index + towel.len() < possible.len() {
                    possible[index + towel.len()] += possible[index];
                }
            }
        }

        result += possible[line.len()];
    }

    println!("{result}");
}
