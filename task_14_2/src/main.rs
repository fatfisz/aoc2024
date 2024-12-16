use std::fs::read_to_string;

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();

    let mut robots = input
        .split('\n')
        .map(|robot| {
            let robot = &robot[2..];
            let comma_index = robot.find(',').unwrap();
            let p_x = robot[0..comma_index].parse::<usize>().unwrap();

            let robot = &robot[comma_index + 1..];
            let space_index = robot.find(' ').unwrap();
            let p_y = robot[0..space_index].parse::<usize>().unwrap();

            let robot = &robot[space_index + 3..];
            let comma_index = robot.find(',').unwrap();
            let v_x = robot[0..comma_index].parse::<isize>().unwrap();

            let robot = &robot[comma_index + 1..];
            let v_y = robot.parse::<isize>().unwrap();

            (p_x, p_y, v_x, v_y)
        })
        .collect::<Vec<_>>();

    let mut iteration = 0;

    while iteration < 6644 {
        iterate(&mut robots);
        iteration += 1;
    }

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{iteration}");
    print(&robots);
}

fn iterate(robots: &mut Vec<(usize, usize, isize, isize)>) {
    for robot in robots {
        robot.0 = (robot.0 + (WIDTH as isize + robot.2) as usize) % WIDTH;
        robot.1 = (robot.1 + (HEIGHT as isize + robot.3) as usize) % HEIGHT;
    }
}

fn print(robots: &Vec<(usize, usize, isize, isize)>) {
    let mut img = vec![vec![b'.'; WIDTH as usize]; HEIGHT as usize];
    for robot in robots {
        img[robot.1][robot.0] = b'x';
    }

    for line in img {
        println!("{}", String::from_utf8(line).unwrap());
    }
}
