use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim().bytes().map(|b| (b - b'0') as usize);

    let mut file_pos_and_sizes = vec![];
    let mut empty_pos_and_sizes = vec![];
    let mut pos = 0;
    for (index, i) in input.enumerate() {
        if index % 2 == 0 {
            file_pos_and_sizes.push((pos, i));
        } else {
            empty_pos_and_sizes.push((pos, i));
        }
        pos += i;
    }

    for file in file_pos_and_sizes.iter_mut().rev() {
        for empty in &mut empty_pos_and_sizes {
            if empty.0 > file.0 {
                break;
            }
            if empty.1 >= file.1 {
                file.0 = empty.0;
                empty.0 += file.1;
                empty.1 -= file.1;
            }
        }
    }

    let result = file_pos_and_sizes
        .iter()
        .enumerate()
        .map(|(index, &(pos, size))| block_checksum(pos, size, index))
        .sum::<usize>();

    println!("{result}");
}

fn block_checksum(pos: usize, size: usize, id: usize) -> usize {
    if size == 0 {
        0
    } else {
        id * (size * pos + size * (size - 1) / 2)
    }
}
