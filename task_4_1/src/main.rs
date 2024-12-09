use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut result = 0;

    let width = input.find('\n').unwrap();
    let input = &input.chars().filter(|&c| c != '\n').collect::<Vec<_>>()[..];
    let height = input.len() / width;

    let from_index = |index: usize| (index % width, index / width);
    let to_index = |x: usize, mod_x: isize, y: usize, mod_y: isize| {
        x.overflowing_add_signed(mod_x).0 + y.overflowing_add_signed(mod_y).0 * width
    };
    let matches = |index: usize, mod_x: isize, mod_y: isize| -> bool {
        let (x, y) = from_index(index);
        (mod_x >= 0 || x >= 3)
            && (mod_x <= 0 || x <= width - 4)
            && (mod_y >= 0 || y >= 3)
            && (mod_y <= 0 || y <= height - 4)
            && input[to_index(x, 0, y, 0)] == 'X'
            && input[to_index(x, mod_x, y, mod_y)] == 'M'
            && input[to_index(x, mod_x * 2, y, mod_y * 2)] == 'A'
            && input[to_index(x, mod_x * 3, y, mod_y * 3)] == 'S'
    };

    for index in 0..input.len() {
        for mod_x in -1..=1 {
            for mod_y in -1..=1 {
                if (mod_x != 0 || mod_y != 0) && matches(index, mod_x, mod_y) {
                    result += 1;
                }
            }
        }
    }

    println!("{result}");
}
