use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut cache = HashMap::<(u128, u8), u64>::new();
    let result = input
        .trim()
        .split(' ')
        .map(|n| mapper(&mut cache, &(n.parse::<u128>().unwrap(), 75)))
        .sum::<u64>();

    println!("{result}");
}

fn mapper(cache: &mut HashMap<(u128, u8), u64>, &(number, iterations_left): &(u128, u8)) -> u64 {
    if iterations_left == 0 {
        1
    } else {
        if !cache.contains_key(&(number, iterations_left)) {
            let result = if number == 0 {
                mapper(cache, &(1, iterations_left - 1))
            } else if number.ilog10() % 2 == 1 {
                let divider = 10_u128.pow(number.ilog10() / 2 + 1);
                mapper(cache, &(number / divider, iterations_left - 1))
                    + mapper(cache, &(number % divider, iterations_left - 1))
            } else {
                mapper(cache, &(number * 2024, iterations_left - 1))
            };

            cache.insert((number, iterations_left), result);
        }

        *cache.get(&(number, iterations_left)).unwrap()
    }
}
