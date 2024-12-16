use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input
        .trim()
        .split('\n')
        .map(|line| line.bytes().map(|b| (b - b'0')).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = input[0].len();
    let height = input.len();

    let mut result = 0;

    for start_y in 0..height {
        for start_x in 0..width {
            if input[start_y][start_x] != 0 {
                continue;
            }

            let mut queue = HashSet::new();
            queue.insert((start_x, start_y));

            for elevation in 1..=9 {
                let mut next_queue = HashSet::new();
                for (x, y) in queue {
                    if x > 0 && input[y][x - 1] == elevation {
                        next_queue.insert((x - 1, y));
                    }
                    if x < width - 1 && input[y][x + 1] == elevation {
                        next_queue.insert((x + 1, y));
                    }
                    if y > 0 && input[y - 1][x] == elevation {
                        next_queue.insert((x, y - 1));
                    }
                    if y < height - 1 && input[y + 1][x] == elevation {
                        next_queue.insert((x, y + 1));
                    }
                }
                queue = next_queue;
            }
            result += queue.len();
        }
    }

    println!("{result}");
}
