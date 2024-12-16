use std::{collections::HashMap, fs::read_to_string};

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

            let mut queue = HashMap::new();
            queue.insert((start_x, start_y), 1);

            for elevation in 1..=9 {
                let mut next_queue = HashMap::new();
                for ((x, y), rating) in queue {
                    if x > 0 && input[y][x - 1] == elevation {
                        inc_by(&mut next_queue, (x - 1, y), rating);
                    }
                    if x < width - 1 && input[y][x + 1] == elevation {
                        inc_by(&mut next_queue, (x + 1, y), rating);
                    }
                    if y > 0 && input[y - 1][x] == elevation {
                        inc_by(&mut next_queue, (x, y - 1), rating);
                    }
                    if y < height - 1 && input[y + 1][x] == elevation {
                        inc_by(&mut next_queue, (x, y + 1), rating);
                    }
                }
                queue = next_queue;
            }
            result += queue.values().sum::<i32>();
        }
    }

    println!("{result}");
}

fn inc_by(next_queue: &mut HashMap<(usize, usize), i32>, pos: (usize, usize), rating: i32) {
    if let Some(prev_rating) = next_queue.get_mut(&pos) {
        *prev_rating += rating;
    } else {
        next_queue.insert(pos, rating);
    }
}
