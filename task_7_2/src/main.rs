use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut result: u64 = 0;

    for line in input.split('\n') {
        let mut line_iter = line.split(": ");
        let head: u64 = line_iter.next().unwrap().parse().unwrap();
        let tail: Vec<_> = line_iter
            .next()
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        for mut mask in 0..3_usize.pow(tail.len() as u32) {
            let mut mask_result = tail[0];

            for number in tail[1..].iter() {
                match mask % 3 {
                    0 => mask_result += number,
                    1 => mask_result *= number,
                    2 => mask_result = mask_result * 10_u64.pow(number.ilog10() + 1) + number,
                    _ => {}
                }
                if mask_result > head {
                    break;
                }
                mask /= 3;
            }
            if mask_result == head {
                result += head;
                break;
            }
        }
    }

    println!("{result}");
}
