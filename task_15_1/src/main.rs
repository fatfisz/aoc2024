use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();

    let mut input_iter = input.split("\n\n");
    let map = input_iter.next().unwrap();
    let moves = input_iter.next().unwrap();

    let mut map = map
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut robot_pos = (0, 0);

    'outer: for (y, line) in map.iter_mut().enumerate() {
        for (x, character) in line.iter_mut().enumerate() {
            if *character == '@' {
                *character = '.';
                robot_pos = (x, y);
                break 'outer;
            }
        }
    }

    for robot_move in moves.chars() {
        let next_pos = apply_move(robot_pos, robot_move);
        match map[next_pos.1][next_pos.0] {
            '.' => {
                robot_pos = next_pos;
            }
            'O' => {
                let mut empty_pos = next_pos;
                loop {
                    empty_pos = apply_move(empty_pos, robot_move);
                    match map[empty_pos.1][empty_pos.0] {
                        '.' => {
                            robot_pos = next_pos;
                            map[next_pos.1][next_pos.0] = '.';
                            map[empty_pos.1][empty_pos.0] = 'O';
                            break;
                        }
                        '#' => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    let mut result = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, &character) in line.iter().enumerate() {
            if character == 'O' {
                result += x + 100 * y;
            }
        }
    }

    println!("{result}");
}

fn apply_move((x, y): (usize, usize), robot_move: char) -> (usize, usize) {
    match robot_move {
        '^' => (x, y - 1),
        '>' => (x + 1, y),
        'v' => (x, y + 1),
        '<' => (x - 1, y),
        _ => (x, y),
    }
}
