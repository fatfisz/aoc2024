use std::{fs::read_to_string, iter::repeat};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();
    let mut positions: Vec<Vec<Point>> = repeat(vec![]).take(128).collect();

    let frequencies: String = ('0'..='9').chain('A'..='Z').chain('a'..='z').collect();
    let frequencies = frequencies.chars().map(|c| c as usize).collect::<Vec<_>>();

    let mut result = 0;
    let width = input.split('\n').next().unwrap().len();
    let height = input.split('\n').count();

    for (y, line) in input.split('\n').enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character != '.' {
                let _ = &positions[character as usize].push(Point { x, y });
            }
        }
    }

    let positions = positions;
    let mut antinodes = vec![vec![false; width]; height];

    for frequency in frequencies {
        if positions[frequency].len() == 0 {
            continue;
        }
        for (index, first) in positions[frequency].iter().enumerate() {
            for second in positions[frequency][index + 1..].iter() {
                for m in 0.. {
                    if is_ok(first.x, second.x, width, m) && is_ok(first.y, second.y, height, m) {
                        if !antinodes[calc(first.y, second.y, m)][calc(first.x, second.x, m)] {
                            antinodes[calc(first.y, second.y, m)][calc(first.x, second.x, m)] =
                                true;
                            result += 1;
                        }
                    } else {
                        break;
                    }
                }
                for m in 0.. {
                    if is_ok(second.x, first.x, width, m) && is_ok(second.y, first.y, height, m) {
                        if !antinodes[calc(second.y, first.y, m)][calc(second.x, first.x, m)] {
                            antinodes[calc(second.y, first.y, m)][calc(second.x, first.x, m)] =
                                true;
                            result += 1;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("{result}");
}

fn is_ok(first: usize, second: usize, limit: usize, m: usize) -> bool {
    (m + 1) * first >= m * second && (m + 1) * first < limit + m * second
}

fn calc(first: usize, second: usize, m: usize) -> usize {
    (m + 1) * first - m * second
}

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}
