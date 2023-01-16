use std::collections::VecDeque;

pub fn part1(input: &[&str]) -> u32 {
    let mut starting_points = Vec::new();
    let mut end = (0, 0);

    let height_map = parse_map(input, &mut starting_points, &mut end, &[b'S']);

    starting_points
        .into_iter()
        .map(|start| find_shortest_steps(height_map.as_slice(), start, end))
        .min()
        .unwrap()
}

pub fn part2(input: &[&str]) -> u32 {
    let mut starting_points = Vec::new();
    let mut end = (0, 0);

    let height_map = parse_map(input, &mut starting_points, &mut end, &[b'S', b'a']);

    starting_points
        .into_iter()
        .map(|start| find_shortest_steps(height_map.as_slice(), start, end))
        .min()
        .unwrap()
}

fn parse_map<'a>(
    input: &[&'a str],
    starting_points: &mut Vec<(usize, usize)>,
    end: &mut (usize, usize),
    starting_matches: &[u8],
) -> Vec<&'a [u8]> {
    input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let row = row.as_bytes();
            if let Some(x) = row.iter().position(|b| starting_matches.contains(b)) {
                starting_points.push((x, y));
            }
            if let Some(x) = row.iter().position(|&b| b == b'E') {
                *end = (x, y);
            }
            row
        })
        .collect::<Vec<_>>()
}

fn find_shortest_steps(height_map: &[&[u8]], start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut visited = vec![vec![false; height_map[0].len()]; height_map.len()];
    let mut dist = vec![vec![0u32; height_map[0].len()]; height_map.len()];

    let mut queue = VecDeque::new();
    queue.push_back(start);

    visited[start.1][start.0] = true;

    let check_node = |next: (usize, usize),
                      current: (usize, usize),
                      queue: &mut VecDeque<(usize, usize)>,
                      dist: &mut Vec<Vec<u32>>,
                      visited: &mut Vec<Vec<bool>>|
     -> bool {
        let current_height = if height_map[current.1][current.0] == b'S' {
            b'a'
        } else {
            height_map[current.1][current.0]
        };
        let height = if height_map[next.1][next.0] == b'E' {
            b'z'
        } else {
            height_map[next.1][next.0]
        };
        if height <= current_height + 1 {
            dist[next.1][next.0] = dist[current.1][current.0] + 1;
            if next == end {
                return true;
            }
            visited[next.1][next.0] = true;
            queue.push_back(next);
        }
        false
    };

    while let Some(current) = queue.pop_front() {
        let left = (current.0.saturating_sub(1), current.1);
        let right = ((current.0 + 1).min(height_map[0].len() - 1), current.1);
        let up = (current.0, current.1.saturating_sub(1));
        let down = (current.0, (current.1 + 1).min(height_map.len() - 1));

        if !visited[left.1][left.0]
            && check_node(left, current, &mut queue, &mut dist, &mut visited)
        {
            break;
        }
        if !visited[right.1][right.0]
            && check_node(right, current, &mut queue, &mut dist, &mut visited)
        {
            break;
        }
        if !visited[up.1][up.0] && check_node(up, current, &mut queue, &mut dist, &mut visited) {
            break;
        }
        if !visited[down.1][down.0]
            && check_node(down, current, &mut queue, &mut dist, &mut visited)
        {
            break;
        }
    }

    dist[end.1][end.0]
}
