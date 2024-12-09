use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let width = input.split('\n').next().unwrap().len();
    let height = input.split('\n').count();
    let mut obstacles = vec![vec![false; width]; height];

    let mut result = 0;
    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::Up;

    input.split('\n').enumerate().for_each(|(yy, line)| {
        line.chars().enumerate().for_each(|(xx, c)| {
            if c == '^' {
                x = xx;
                y = yy;
            }
            obstacles[yy][xx] = c == '#';
        });
    });

    let mut checked_obstacles = obstacles.clone();
    checked_obstacles[y][x] = true;

    let initial_x = x;
    let initial_y = y;
    let initial_dir = dir;

    loop {
        let Some((next_x, next_y)) = next_pos(width, height, x, y, dir) else {
            break;
        };
        if !checked_obstacles[next_y][next_x] {
            checked_obstacles[next_y][next_x] = true;
            if will_putting_obstacle_next_result_in_a_loop(
                obstacles.clone(),
                width,
                height,
                initial_x,
                initial_y,
                initial_dir,
                next_x,
                next_y,
            ) {
                result += 1;
            }
        }
        if obstacles[next_y][next_x] {
            dir = dir.rotate();
        } else {
            (x, y) = (next_x, next_y);
        }
    }

    println!("{result}");
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn rotate(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

fn will_putting_obstacle_next_result_in_a_loop(
    initial_obstacles: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    initial_x: usize,
    initial_y: usize,
    initial_dir: Dir,
    obstacle_x: usize,
    obstacle_y: usize,
) -> bool {
    let mut obstacles = initial_obstacles;
    obstacles[obstacle_y][obstacle_x] = true;
    let obstacles = obstacles;

    let mut x = initial_x;
    let mut y = initial_y;
    let mut dir = initial_dir;

    let mut steps = 0;
    loop {
        let Some((next_x, next_y)) = next_pos(width, height, x, y, dir) else {
            break;
        };
        if obstacles[next_y][next_x] {
            dir = dir.rotate();
        } else {
            (x, y) = (next_x, next_y);
        }
        steps += 1;
        // If we reach that number, then there's definitely a loop; this could be
        // optimised, but it works alright
        if steps == width * height * 4 {
            return true;
        }
    }

    false
}

fn next_pos(width: usize, height: usize, x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
    match dir {
        Dir::Left => {
            if x == 0 {
                return None;
            } else {
                Some((x - 1, y))
            }
        }
        Dir::Right => {
            if x == width - 1 {
                return None;
            } else {
                Some((x + 1, y))
            }
        }
        Dir::Up => {
            if y == 0 {
                return None;
            } else {
                Some((x, y - 1))
            }
        }
        Dir::Down => {
            if y == height - 1 {
                return None;
            } else {
                Some((x, y + 1))
            }
        }
    }
}
