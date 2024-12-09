use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut safe_count = 0;

    for reports in input.split('\n') {
        let levels: Vec<_> = reports
            .split_whitespace()
            .map(|level| level.parse::<i32>().unwrap())
            .collect();

        let unsafe_index = is_report_safe(&levels);

        let safe = match unsafe_index {
            // This is a special case because of the possible order mismatch, which at
            // index 1 might mean more than in other cases.
            //
            // For example, `is_report_safe(vec![1, 3, 2, 1])` will return `Some(1)`, as
            // 1 < 3, but 3 > 2. Removing 3 yields `vec![1, 2, 1]`, which is unsafe,
            // while removing 1 yields `vec![3, 2, 1]`, a safe report.
            Some(1) => {
                is_report_safe(&report_without_index(&levels, 0)).is_none()
                    || is_report_safe(&report_without_index(&levels, 1)).is_none()
                    || is_report_safe(&report_without_index(&levels, 2)).is_none()
            }
            Some(index) => {
                is_report_safe(&report_without_index(&levels, index)).is_none()
                    || is_report_safe(&report_without_index(&levels, index + 1)).is_none()
            }
            None => true,
        };

        if safe {
            safe_count += 1;
        }
    }

    println!("{safe_count}");
}

fn report_without_index(levels: &[i32], index: usize) -> Vec<i32> {
    [&levels[0..index], &levels[index + 1..]].concat()
}

fn is_report_safe(levels: &Vec<i32>) -> Option<usize> {
    let diff = levels[1] - levels[0];
    if !is_diff_safe(diff) {
        return Some(0);
    }

    let asc = diff > 0;
    let mut last_level = levels[1];

    for (index, level) in levels[2..].iter().enumerate() {
        let diff = *level - last_level;
        if !is_diff_safe(diff) || ((diff > 0) != asc) {
            return Some(index + 1);
        }
        last_level = *level;
    }

    None
}

fn is_diff_safe(diff: i32) -> bool {
    (1..=3).contains(&diff.abs())
}
