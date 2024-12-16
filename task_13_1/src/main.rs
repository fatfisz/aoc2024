use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut result = 0;

    'outer: for machine in input.split("\n\n") {
        let lines = machine.split('\n').collect::<Vec<_>>();

        let a_line = lines[0];
        let b_line = lines[1];
        let prize_line = lines[2];

        let a_x = a_line[a_line.find("X+").unwrap() + 2..a_line.find(",").unwrap()]
            .parse::<usize>()
            .unwrap();
        let a_y = a_line[a_line.find("Y+").unwrap() + 2..]
            .parse::<usize>()
            .unwrap();
        let b_x = b_line[b_line.find("X+").unwrap() + 2..b_line.find(",").unwrap()]
            .parse::<usize>()
            .unwrap();
        let b_y = b_line[b_line.find("Y+").unwrap() + 2..]
            .parse::<usize>()
            .unwrap();
        let prize_x = prize_line[prize_line.find("X=").unwrap() + 2..prize_line.find(",").unwrap()]
            .parse::<usize>()
            .unwrap();
        let prize_y = prize_line[prize_line.find("Y=").unwrap() + 2..]
            .parse::<usize>()
            .unwrap();

        for a in 0..=100 {
            for b in 0..=100 {
                if a * a_x + b * b_x == prize_x && a * a_y + b * b_y == prize_y {
                    result += 3 * a + b;
                    continue 'outer;
                }
            }
        }
    }

    println!("{result}");
}
