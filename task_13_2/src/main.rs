use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut result = 0;

    for machine in input.split("\n\n") {
        let lines = machine.split('\n').collect::<Vec<_>>();

        let a_line = lines[0];
        let b_line = lines[1];
        let prize_line = lines[2];

        let a_x = a_line[a_line.find("X+").unwrap() + 2..a_line.find(",").unwrap()]
            .parse::<i64>()
            .unwrap();
        let a_y = a_line[a_line.find("Y+").unwrap() + 2..]
            .parse::<i64>()
            .unwrap();
        let b_x = b_line[b_line.find("X+").unwrap() + 2..b_line.find(",").unwrap()]
            .parse::<i64>()
            .unwrap();
        let b_y = b_line[b_line.find("Y+").unwrap() + 2..]
            .parse::<i64>()
            .unwrap();
        let prize_x = prize_line[prize_line.find("X=").unwrap() + 2..prize_line.find(",").unwrap()]
            .parse::<i64>()
            .unwrap()
            + 10000000000000;
        let prize_y = prize_line[prize_line.find("Y=").unwrap() + 2..]
            .parse::<i64>()
            .unwrap()
            + 10000000000000;

        let prize_product = prize_x * a_y - prize_y * a_x;
        let button_product = b_x * a_y - a_x * b_y;

        if prize_product % button_product == 0 {
            let b = prize_product / button_product;
            let a = (prize_x - b * b_x) / a_x;
            if (prize_x - b * b_x) % a_x != 0 || a < 0 || b < 0 {
                panic!("Something went wrong");
            }
            result += 3 * a + b;
        }
    }

    println!("{result}");
}
