use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim().split("\n\n");

    let mut locks = HashSet::new();
    let mut keys = HashSet::new();
    let mut space = usize::MAX;

    for input in input {
        let lines = input.split('\n').collect::<Vec<_>>();
        if space == usize::MAX {
            space = lines.len();
        } else if lines.len() != space {
            panic!("Different available space");
        }

        let mut schematic = [0; 5];
        if lines[0] == "#####" {
            for (index, &line) in lines.iter().enumerate() {
                for (char_index, character) in line.chars().enumerate() {
                    if character == '.' && schematic[char_index] == 0 {
                        schematic[char_index] = index;
                    }
                }
            }
            locks.insert(schematic);
        } else {
            for (index, &line) in lines.iter().rev().enumerate() {
                for (char_index, character) in line.chars().enumerate() {
                    if character == '.' && schematic[char_index] == 0 {
                        schematic[char_index] = index;
                    }
                }
            }
            keys.insert(schematic);
        }
    }

    let mut unfit = 0;

    for lock in &locks {
        for key in &keys {
            for index in 0..5 {
                if lock[index] + key[index] > space {
                    unfit += 1;
                    break;
                }
            }
        }
    }

    let result = locks.len() * keys.len() - unfit;

    println!("{result}");
}
