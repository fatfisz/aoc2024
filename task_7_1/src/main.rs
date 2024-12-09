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

        for mut mask in 0..2_usize.pow(tail.len() as u32) {
            let mut mask_result = tail[0];

            for number in tail[1..].iter() {
                if mask % 2 == 1 {
                    mask_result *= number;
                } else {
                    mask_result += number;
                }
                if mask_result > head {
                    break;
                }
                mask >>= 1;
            }
            if mask_result == head {
                result += head;
                break;
            }
        }
    }

    println!("{result}");
}
