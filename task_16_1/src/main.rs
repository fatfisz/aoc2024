use std::{cmp::Ordering, collections::BinaryHeap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();

    let map = input
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = map[0].len();
    let height = map.len();

    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    for (y, line) in map.iter().enumerate() {
        for (x, &character) in line.iter().enumerate() {
            match character {
                'S' => {
                    start_pos = (x, y);
                }
                'E' => {
                    end_pos = (x, y);
                }
                _ => {}
            }
        }
    }

    let start_pos = start_pos;
    let end_pos = end_pos;

    let mut min_dist = map
        .iter()
        .map(|line| {
            line.iter()
                .map(|&character| {
                    [
                        if character == 'S' { 0 } else { usize::MAX },
                        usize::MAX,
                        usize::MAX,
                        usize::MAX,
                    ]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut queue = BinaryHeap::<PositionScore>::new();
    queue.push(PositionScore(start_pos, 0, 0));

    let move_cost = 1;
    let turn_cost = 1000;

    while let Some(PositionScore(pos, dir, cost)) = queue.pop() {
        if cost > min_dist[pos.1][pos.0][dir] {
            continue;
        }

        let next_turn_cost = min_dist[pos.1][pos.0][dir] + turn_cost;
        let next_dir = (dir + 3) % 4;
        if next_turn_cost < min_dist[pos.1][pos.0][next_dir] {
            min_dist[pos.1][pos.0][next_dir] = next_turn_cost;
            queue.push(PositionScore(pos, next_dir, next_turn_cost));
        }

        let next_dir = (dir + 1) % 4;
        if next_turn_cost < min_dist[pos.1][pos.0][next_dir] {
            min_dist[pos.1][pos.0][next_dir] = next_turn_cost;
            queue.push(PositionScore(pos, next_dir, next_turn_cost));
        }

        if let Some(next_pos) = match dir {
            0 => {
                if pos.0 + 1 >= width || map[pos.1][pos.0 + 1] == '#' {
                    None
                } else {
                    Some((pos.0 + 1, pos.1))
                }
            }
            1 => {
                if pos.1 + 1 >= height || map[pos.1 + 1][pos.0] == '#' {
                    None
                } else {
                    Some((pos.0, pos.1 + 1))
                }
            }
            2 => {
                if pos.0 == 0 || map[pos.1][pos.0 - 1] == '#' {
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                }
            }
            3 => {
                if pos.1 == 0 || map[pos.1 - 1][pos.0] == '#' {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                }
            }
            _ => None,
        } {
            let next_move_cost = min_dist[pos.1][pos.0][dir] + move_cost;
            if next_move_cost < min_dist[next_pos.1][next_pos.0][dir] {
                min_dist[next_pos.1][next_pos.0][dir] = next_move_cost;
                queue.push(PositionScore(next_pos, dir, next_move_cost));
            }
        }
    }

    let result = min_dist[end_pos.1][end_pos.0].iter().min().unwrap();

    println!("{result}");
}

struct PositionScore((usize, usize), usize, usize);

impl Eq for PositionScore {}

impl Ord for PositionScore {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&other.2, &self.2)
    }
}

impl PartialEq for PositionScore {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&other.2, &self.2)
    }
}

impl PartialOrd for PositionScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&other.2, &self.2)
    }
}
