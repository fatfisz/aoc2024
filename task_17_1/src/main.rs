use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim().split('\n').collect::<Vec<_>>();

    let mut reg_a = input[0]["Register A: ".len()..].parse::<usize>().unwrap();
    let mut reg_b = input[1]["Register B: ".len()..].parse::<usize>().unwrap();
    let mut reg_c = input[2]["Register C: ".len()..].parse::<usize>().unwrap();
    let program = input[4]["Program: ".len()..]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut instruction_pointer = 0;
    let mut comma = "";

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];

        match opcode {
            0 => reg_a >>= get_combo(operand, reg_a, reg_b, reg_c),
            1 => reg_b ^= operand,
            2 => reg_b = get_combo(operand, reg_a, reg_b, reg_c) & 7,
            3 => {
                if reg_a != 0 {
                    instruction_pointer = operand;
                    continue;
                }
            }
            4 => reg_b ^= reg_c,
            5 => {
                print!("{}{}", comma, get_combo(operand, reg_a, reg_b, reg_c) & 7);
                comma = ",";
            }
            6 => reg_b = reg_a >> get_combo(operand, reg_a, reg_b, reg_c),
            7 => reg_c = reg_a >> get_combo(operand, reg_a, reg_b, reg_c),
            _ => panic!("Invalid opcode"),
        }

        instruction_pointer += 2;
    }

    if comma.len() > 0 {
        println!("");
    }
}

fn get_combo(operand: usize, reg_a: usize, reg_b: usize, reg_c: usize) -> usize {
    match operand {
        0..=3 => operand,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => panic!("Invalid combo operand"),
    }
}
