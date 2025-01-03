use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();

    let mut input_iter = input.split("\n\n");
    let map = input_iter.next().unwrap();
    let moves = input_iter.next().unwrap();

    let mut map = map
        .split('\n')
        .map(|line| {
            line.chars()
                .flat_map(|character| match character {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => panic!("Unknown character"),
                })
                .collect::<Vec<_>>()
        })
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
            '[' | ']' => {
                robot_pos = move_box_stack(&mut map, robot_pos, robot_move, next_pos);
            }
            _ => {}
        }
    }

    let mut result = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, &character) in line.iter().enumerate() {
            if character == '[' {
                result += x + 100 * y;
            }
        }
    }

    println!("{result}");
}

fn move_box_stack(
    map: &mut Vec<Vec<char>>,
    robot_pos: (usize, usize),
    robot_move: char,
    next_pos: (usize, usize),
) -> (usize, usize) {
    let mut box_positions = vec![normalize_box_pos(map, next_pos)];
    let mut current_index = 0;

    while current_index < box_positions.len() {
        let box_pos = box_positions[current_index];

        for next_box_pos in apply_move_to_box(box_pos, robot_move) {
            match map[next_box_pos.1][next_box_pos.0] {
                '#' => {
                    return robot_pos;
                }
                '[' => {
                    box_positions.push(next_box_pos);
                    break;
                }
                ']' => {
                    box_positions.push(normalize_box_pos(map, next_box_pos));
                }
                _ => {}
            }
        }

        current_index += 1;
    }

    for &(box_x, box_y) in box_positions.iter().rev() {
        map[box_y][box_x..][0..2].copy_from_slice(&['.', '.']);
        let (box_x, box_y) = apply_move((box_x, box_y), robot_move);
        map[box_y][box_x..][0..2].copy_from_slice(&['[', ']']);
    }

    return apply_move(robot_pos, robot_move);
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

fn apply_move_to_box((x, y): (usize, usize), robot_move: char) -> Vec<(usize, usize)> {
    match robot_move {
        '^' => vec![(x, y - 1), (x + 1, y - 1)],
        '>' => vec![(x + 2, y)],
        'v' => vec![(x, y + 1), (x + 1, y + 1)],
        '<' => vec![(x - 1, y)],
        _ => vec![],
    }
}

fn normalize_box_pos(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> (usize, usize) {
    match map[y][x] {
        '[' => (x, y),
        ']' => (x - 1, y),
        _ => panic!("Invalid box character"),
    }
}
