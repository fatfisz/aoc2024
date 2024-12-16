use std::fs::read_to_string;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const ITERATIONS: i32 = 100;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();

    let mut q = (0, 0, 0, 0);

    for robot in input.split('\n') {
        let robot = &robot[2..];
        let comma_index = robot.find(',').unwrap();
        let p_x = robot[0..comma_index].parse::<i32>().unwrap();

        let robot = &robot[comma_index + 1..];
        let space_index = robot.find(' ').unwrap();
        let p_y = robot[0..space_index].parse::<i32>().unwrap();

        let robot = &robot[space_index + 3..];
        let comma_index = robot.find(',').unwrap();
        let v_x = robot[0..comma_index].parse::<i32>().unwrap();

        let robot = &robot[comma_index + 1..];
        let v_y = robot.parse::<i32>().unwrap();

        let p_x = ((p_x + v_x * ITERATIONS) % WIDTH + WIDTH) % WIDTH;
        let p_y = ((p_y + v_y * ITERATIONS) % HEIGHT + HEIGHT) % HEIGHT;

        if p_x < WIDTH / 2 {
            if p_y < HEIGHT / 2 {
                q.0 += 1;
            } else if p_y > HEIGHT / 2 {
                q.1 += 1;
            }
        } else if p_x > WIDTH / 2 {
            if p_y < HEIGHT / 2 {
                q.2 += 1;
            } else if p_y > HEIGHT / 2 {
                q.3 += 1;
            }
        }
    }

    let result = q.0 * q.1 * q.2 * q.3;

    println!("{result}");
}
