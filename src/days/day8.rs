pub fn part1(input: &[&str]) -> u32 {
    let bytes = input.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();

    let mut visible = 0;

    for (y, row) in bytes.iter().enumerate() {
        if y == 0 || y == bytes.len() - 1 {
            visible += row.len() as u32;
            continue;
        }
        for (x, num) in row.iter().enumerate() {
            if x == 0
                || x == row.len() - 1
                || row[..x].iter().all(|i| i < num)
                || row[(x + 1)..].iter().all(|i| i < num)
                || bytes.iter().take(y).all(|row2| row2[x] < *num)
                || bytes.iter().skip(y + 1).all(|row2| row2[x] < *num)
            {
                visible += 1;
            }
        }
    }

    visible
}

pub fn part2(input: &[&str]) -> usize {
    let bytes = input.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();

    let mut max_view_distance = 0;

    for (y, row) in bytes.iter().enumerate() {
        if y == 0 || y == bytes.len() - 1 {
            continue;
        }
        for (x, num) in row.iter().enumerate() {
            if x == 0 || x == row.len() - 1 {
                continue;
            }
            let mut view_distance = 1;
            if let Some(x2) = row[..x].iter().rposition(|i| i >= num) {
                view_distance *= x - x2;
            } else {
                view_distance *= x;
            }
            if let Some(x2) = row[(x + 1)..].iter().position(|i| i >= num) {
                view_distance *= x2 + 1;
            } else {
                view_distance *= row.len() - 1 - x;
            }
            if let Some(y2) = bytes[..y].iter().rposition(|row2| row2[x] >= *num) {
                view_distance *= y - y2;
            } else {
                view_distance *= y;
            }
            if let Some(y2) = bytes[(y + 1)..].iter().position(|row2| row2[x] >= *num) {
                view_distance *= y2 + 1;
            } else {
                view_distance *= bytes.len() - 1 - y;
            }
            max_view_distance = max_view_distance.max(view_distance);
        }
    }

    max_view_distance
}

#[cfg(test)]
mod tests {
    use crate::{day8, parse_input_static};

    #[test]
    fn test_part1() {
        let input = parse_input_static(
            "30373
25512
65332
33549
35390",
        );

        assert_eq!(21, day8::part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input_static(
            "30373
25512
65332
33549
35390",
        );

        assert_eq!(8, day8::part2(&input));
    }
}
