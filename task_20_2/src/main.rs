use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();

    let input = input
        .split('\n')
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = input[0].len();
    let height = input.len();
    let max_dist = width * height;

    let mut visited = input
        .iter()
        .map(|line| line.iter().map(|&b| b == b'#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut dist = input
        .iter()
        .map(|line| [max_dist].repeat(line.len()))
        .collect::<Vec<_>>();

    let mut start_pos = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if input[y][x] == b'S' {
                start_pos = (x, y);
            }
        }
    }

    let mut queue = vec![start_pos];
    let mut queue_index = 0;
    dist[start_pos.1][start_pos.0] = 0;
    visited[start_pos.1][start_pos.0] = true;

    while queue_index < queue.len() {
        let (x, y) = queue[queue_index];

        if !visited[y][x - 1] {
            dist[y][x - 1] = dist[y][x] + 1;
            visited[y][x - 1] = true;
            queue.push((x - 1, y));
        }
        if !visited[y][x + 1] {
            dist[y][x + 1] = dist[y][x] + 1;
            visited[y][x + 1] = true;
            queue.push((x + 1, y));
        }
        if !visited[y - 1][x] {
            dist[y - 1][x] = dist[y][x] + 1;
            visited[y - 1][x] = true;
            queue.push((x, y - 1));
        }
        if !visited[y + 1][x] {
            dist[y + 1][x] = dist[y][x] + 1;
            visited[y + 1][x] = true;
            queue.push((x, y + 1));
        }

        queue_index += 1;
    }

    let mut result = 0;
    let cheat_limit = 20;
    let threshold = 100;

    for y1 in 0..height {
        for x1 in 0..width {
            if dist[y1][x1] == max_dist {
                continue;
            }

            let min_y2 = y1.max(cheat_limit) - cheat_limit;
            let max_y2 = (y1 + cheat_limit + 1).min(height);

            for y2 in min_y2..max_y2 {
                let dist_y = y1.abs_diff(y2);

                let min_x2 = (x1 + dist_y).max(cheat_limit) - cheat_limit;
                let max_x2 = (x1 + cheat_limit + 1 - dist_y).min(height);

                for x2 in min_x2..max_x2 {
                    let dist_x = x1.abs_diff(x2);
                    let cheat_dist = dist_x + dist_y;

                    if dist[y2][x2] < max_dist
                        && dist[y2][x2] >= dist[y1][x1] + cheat_dist + threshold
                    {
                        result += 1;
                    }
                }
            }
        }
    }

    println!("{result}");
}
