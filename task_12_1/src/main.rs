use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let width = input.find('\n').unwrap();
    let height = input.len() / (width + 1);

    let input = input
        .split('\n')
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut plots = (0..width * height)
        .map(|id| Plot {
            id,
            parent: id,
            area: 1,
            perimeter: 4,
        })
        .collect::<Vec<_>>();

    for index in 0..plots.len() {
        if index >= width && input[index] == input[index - width] {
            union(&mut plots, index, index - width);
        }
        if index % width >= 1 && input[index] == input[index - 1] {
            union(&mut plots, index, index - 1);
        }
    }

    let mut result: usize = 0;

    for index in 0..plots.len() {
        find(&mut plots, index);
        if plots[index].parent == index {
            result += plots[index].area * plots[index].perimeter;
        }
    }

    println!("{result}");
}

fn union(plots: &mut Vec<Plot>, this_index: usize, other_index: usize) {
    let this_id = find(plots, this_index);
    let other_id = find(plots, other_index);

    if this_id == other_id {
        plots[this_id].perimeter -= 2;
    } else if plots[this_id].area >= plots[other_id].area {
        plots[other_id].parent = this_id;
        plots[this_id].area += plots[other_id].area;
        plots[this_id].perimeter += plots[other_id].perimeter - 2;
    } else {
        plots[this_id].parent = other_id;
        plots[other_id].area += plots[this_id].area;
        plots[other_id].perimeter += plots[this_id].perimeter - 2;
    }
}

fn find(plots: &mut Vec<Plot>, index: usize) -> usize {
    let mut plot_id = plots[index].id;
    let mut root_id = plot_id;

    while plots[root_id].id != plots[root_id].parent {
        root_id = plots[root_id].parent;
    }

    while plot_id != root_id {
        let parent_id = plots[plot_id].parent;
        plots[plot_id].parent = root_id;
        plot_id = parent_id;
    }

    root_id
}

#[derive(Debug)]
struct Plot {
    id: usize,
    parent: usize,
    area: usize,
    perimeter: usize,
}
