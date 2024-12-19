use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();

    let size = 71;

    let mut visited = vec![vec![false; size]; size];
    for line in input.split('\n').take(1024) {
        let mut iter = line.split(',');
        let x = iter.next().and_then(|s| s.parse::<usize>().ok()).unwrap();
        let y = iter.next().and_then(|s| s.parse::<usize>().ok()).unwrap();

        visited[y][x] = true;
    }
    visited[0][0] = true;

    let mut queue = vec![((0, 0), 0)];
    let mut queue_index = 0;
    let end = (size - 1, size - 1);

    while queue_index < queue.len() {
        let (pos, dist) = queue[queue_index];
        if pos == end {
            println!("{dist}");
            break;
        }

        let (x, y) = pos;
        if x > 0 && !visited[y][x - 1] {
            queue.push(((x - 1, y), dist + 1));
            visited[y][x - 1] = true;
        }
        if x < size - 1 && !visited[y][x + 1] {
            queue.push(((x + 1, y), dist + 1));
            visited[y][x + 1] = true;
        }
        if y > 0 && !visited[y - 1][x] {
            queue.push(((x, y - 1), dist + 1));
            visited[y - 1][x] = true;
        }
        if y < size - 1 && !visited[y + 1][x] {
            queue.push(((x, y + 1), dist + 1));
            visited[y + 1][x] = true;
        }

        queue_index += 1;
    }
}
