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
        input[to_index(x, -mod_x, y, -mod_y)] == 'M'
            && input[to_index(x, 0, y, 0)] == 'A'
            && input[to_index(x, mod_x, y, mod_y)] == 'S'
    };

    for index in 0..input.len() {
        let (x, y) = from_index(index);
        if x > 0
            && x < width - 1
            && y > 0
            && y < height - 1
            && (matches(index, 1, 1) || matches(index, -1, -1))
            && (matches(index, -1, 1) || matches(index, 1, -1))
        {
            result += 1;
        }
    }

    println!("{result}");
}
