use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim().bytes().map(|b| (b - b'0') as usize);
    let mut ids = input.clone().step_by(2).collect::<Vec<usize>>();
    let mut empty = input.skip(1).step_by(2).collect::<Vec<usize>>();

    let mut ids_index = 0;
    let mut rev_ids_index = ids.len() - 1;
    let mut empty_index = 0;
    let mut pos = 0;
    let mut result = 0;

    loop {
        result += block_checksum(pos, ids[ids_index], ids_index);
        if ids_index >= rev_ids_index {
            break;
        }

        pos += ids[ids_index];
        ids[ids_index] = 0;
        ids_index += 1;

        while empty[empty_index] > 0 {
            let block_size = empty[empty_index].min(ids[rev_ids_index]);
            result += block_checksum(pos, block_size, rev_ids_index);
            pos += block_size;
            ids[rev_ids_index] -= block_size;
            empty[empty_index] -= block_size;

            if ids[rev_ids_index] == 0 {
                rev_ids_index -= 1;
            }
        }
        empty_index += 1;
    }

    println!("{result}");
}

fn block_checksum(pos: usize, size: usize, id: usize) -> usize {
    if size == 0 {
        0
    } else {
        id * (size * pos + size * (size - 1) / 2)
    }
}
