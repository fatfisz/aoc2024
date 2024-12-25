use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input
        .trim()
        .split('\n')
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let result = input
        .iter()
        .map(|&n| {
            let mut secret = n;
            for _ in 0..2000 {
                secret = next_secret(secret);
            }
            secret
        })
        .sum::<u64>();

    println!("{result}");
}

const PRUNE_MASK: u64 = (1 << 24) - 1;

fn next_secret(secret: u64) -> u64 {
    let secret = secret ^ (secret << 6) & PRUNE_MASK;
    let secret = secret ^ (secret >> 5) & PRUNE_MASK;
    secret ^ (secret << 11) & PRUNE_MASK
}
