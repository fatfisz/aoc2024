use std::{fs::read_to_string, time::Instant};

fn main() {
    let now = Instant::now();

    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();

    let size = 71;

    let mut corrupted_indexes = vec![vec![usize::MAX; size]; size];
    let mut corrupteds = vec![];
    corrupteds.reserve(size * size);

    'outer: for line in input.split('\n') {
        let [x, y] = line
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()[0..]
        else {
            panic!("Invalid line");
        };

        let mut index = corrupteds.len();
        let corrupted = Corrupted {
            index,
            parent_index: index,
            size: 1,
            left_or_bottom: x == 0 || y == size - 1,
            top_or_right: y == 0 || x == size - 1,
        };
        let mut left_or_bottom = corrupted.left_or_bottom;
        let mut top_or_right = corrupted.top_or_right;
        corrupteds.push(corrupted);

        for n_y in if y > 0 { y - 1 } else { y }..=if y < size - 1 { y + 1 } else { y } {
            for n_x in if x > 0 { x - 1 } else { x }..=if x < size - 1 { x + 1 } else { x } {
                let other_index = corrupted_indexes[n_y][n_x];
                if other_index < usize::MAX {
                    let other_index = find(&mut corrupteds, other_index);
                    if index == other_index {
                        corrupted_indexes[n_y][n_x] = index;
                        continue;
                    }

                    left_or_bottom = left_or_bottom || corrupteds[other_index].left_or_bottom;
                    top_or_right = top_or_right || corrupteds[other_index].top_or_right;

                    if left_or_bottom && top_or_right {
                        println!("{x},{y}");
                        break 'outer;
                    }

                    index = union(&mut corrupteds, index, other_index);
                    corrupteds[index].left_or_bottom = left_or_bottom;
                    corrupteds[index].top_or_right = top_or_right;
                    corrupted_indexes[n_y][n_x] = index;
                }
            }
        }

        corrupted_indexes[y][x] = index;
    }

    println!("{:.2?}", now.elapsed());
}

#[derive(Debug)]
struct Corrupted {
    index: usize,
    parent_index: usize,
    size: usize,
    left_or_bottom: bool,
    top_or_right: bool,
}

impl FindUnionable for Corrupted {
    fn index(&self) -> usize {
        self.index
    }

    fn parent_index(&self) -> usize {
        self.parent_index
    }

    fn set_parent_index(&mut self, parent_indedx: usize) {
        self.parent_index = parent_indedx;
    }

    fn size(&self) -> usize {
        self.size
    }

    fn inc_size(&mut self, size: usize) {
        self.size += size;
    }
}

trait FindUnionable {
    fn index(&self) -> usize;
    fn parent_index(&self) -> usize;
    fn set_parent_index(&mut self, parent_index: usize);
    fn size(&self) -> usize;
    fn inc_size(&mut self, size: usize);
}

fn union<Value: FindUnionable>(
    find_unionable: &mut Vec<Value>,
    self_index: usize,
    other_index: usize,
) -> usize {
    let self_index = find(find_unionable, self_index);
    let other_index = find(find_unionable, other_index);

    if self_index == other_index {
        self_index
    } else if find_unionable[self_index].size() >= find_unionable[other_index].size() {
        find_unionable[other_index].set_parent_index(self_index);
        let other_size = find_unionable[other_index].size();
        find_unionable[self_index].inc_size(other_size);
        self_index
    } else {
        find_unionable[self_index].set_parent_index(other_index);
        let self_size = find_unionable[self_index].size();
        find_unionable[other_index].inc_size(self_size);
        other_index
    }
}

fn find<Value: FindUnionable>(find_unionable: &mut Vec<Value>, index: usize) -> usize {
    let mut root_index = index;
    while find_unionable[root_index].index() != find_unionable[root_index].parent_index() {
        root_index = find_unionable[root_index].parent_index();
    }

    let mut index = index;
    while index != root_index {
        let parent_index = find_unionable[index].parent_index();
        find_unionable[index].set_parent_index(root_index);
        index = parent_index;
    }

    root_index
}
