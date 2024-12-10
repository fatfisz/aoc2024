use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut safe_count = 0;

    'outer: for reports in input.split('\n') {
        let levels: Vec<_> = reports
            .split_whitespace()
            .map(|level| level.parse::<i32>().unwrap())
            .collect();

        let mut last_level = levels[0];
        let asc = levels[1] > levels[0];

        for &level in &levels[1..] {
            let diff = level - last_level;
            if !is_safe(diff) || ((diff > 0) != asc) {
                continue 'outer;
            }
            last_level = level;
        }

        safe_count += 1;
    }

    println!("{safe_count}");
}

fn is_safe(diff: i32) -> bool {
    (1..=3).contains(&diff.abs())
}
