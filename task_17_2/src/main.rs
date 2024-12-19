use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim().split('\n').collect::<Vec<_>>();

    let program = input[4]["Program: ".len()..]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut initial_reg_a_queue = vec![0];
    let mut queue_index = 0;

    let mut result = usize::MAX;

    while queue_index < initial_reg_a_queue.len() {
        let initial_initial_reg_a = initial_reg_a_queue[queue_index];

        for initial_reg_a in initial_initial_reg_a..initial_initial_reg_a + 8 {
            let mut reg_a = initial_reg_a;
            let mut reg_b = 0;
            let mut reg_c = 0;

            let mut instruction_pointer = 0;
            let mut outs = vec![];

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
                        let out = get_combo(operand, reg_a, reg_b, reg_c) & 7;
                        outs.push(out);
                    }
                    6 => reg_b = reg_a >> get_combo(operand, reg_a, reg_b, reg_c),
                    7 => reg_c = reg_a >> get_combo(operand, reg_a, reg_b, reg_c),
                    _ => panic!("Invalid opcode"),
                }

                instruction_pointer += 2;
            }

            if outs.len() <= program.len() && program.ends_with(&outs) {
                if outs.len() == program.len() {
                    result = result.min(initial_reg_a);
                }
                initial_reg_a_queue.push(initial_reg_a * 8);
            }
        }

        queue_index += 1;
    }

    println!("{result}");
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
