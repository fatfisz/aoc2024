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
                if (is_ok(first.x, second.x, width))
                    && (is_ok(first.y, second.y, height))
                    && !antinodes[calc(first.y, second.y)][calc(first.x, second.x)]
                {
                    antinodes[calc(first.y, second.y)][calc(first.x, second.x)] = true;
                    result += 1;
                }
                if (is_ok(second.x, first.x, width))
                    && (is_ok(second.y, first.y, height))
                    && !antinodes[calc(second.y, first.y)][calc(second.x, first.x)]
                {
                    antinodes[calc(second.y, first.y)][calc(second.x, first.x)] = true;
                    result += 1;
                }
            }
        }
    }

    println!("{result}");
}

fn is_ok(first: usize, second: usize, limit: usize) -> bool {
    2 * first >= second && 2 * first < limit + second
}

fn calc(first: usize, second: usize) -> usize {
    2 * first - second
}

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}
