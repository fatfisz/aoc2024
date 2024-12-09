use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let width = input.split('\n').next().unwrap().len();
    let height = input.split('\n').count();
    let mut obstacle = vec![vec![false; width]; height];
    let mut visited = vec![vec![false; width]; height];

    let mut result = 1;
    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::Up;

    input.split('\n').enumerate().for_each(|(yy, line)| {
        line.chars().enumerate().for_each(|(xx, c)| {
            if c == '^' {
                x = xx;
                y = yy;
                visited[yy][xx] = true;
            }
            obstacle[yy][xx] = c == '#';
        });
    });

    loop {
        let Some((next_x, next_y)) = next_pos(width, height, x, y, dir) else {
            break;
        };
        if obstacle[next_y][next_x] {
            dir = dir.rotate();
        } else {
            (x, y) = (next_x, next_y);
            if !visited[y][x] {
                result += 1;
                visited[y][x] = true;
            }
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
