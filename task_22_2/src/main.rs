use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input
        .trim()
        .split('\n')
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut seq_sums = HashMap::new();

    for n in input {
        let mut seq_to_price = HashMap::new();
        let mut secret = n;
        let mut prev_digit = (secret % 10) as u32;
        let mut prev_seq: u32 = 0;
        for index in 0..2000 {
            secret = next_secret(secret);
            let secret_digit = (secret % 10) as u32;
            let diff = if secret_digit > prev_digit {
                secret_digit - prev_digit
            } else {
                10 + prev_digit - secret_digit
            };
            let seq = (prev_seq << 8) + diff;
            if index >= 3 {
                seq_to_price.entry(seq).or_insert(secret_digit);
            }
            prev_digit = secret_digit;
            prev_seq = seq;
        }
        for (seq, price) in seq_to_price {
            seq_sums
                .entry(seq)
                .and_modify(|prev| {
                    *prev += price;
                })
                .or_insert(price);
        }
    }

    let result = seq_sums.values().max().unwrap();
    println!("{result}");
}

const PRUNE_MASK: u64 = (1 << 24) - 1;

fn next_secret(secret: u64) -> u64 {
    let secret = secret ^ (secret << 6) & PRUNE_MASK;
    let secret = secret ^ (secret >> 5) & PRUNE_MASK;
    secret ^ (secret << 11) & PRUNE_MASK
}
