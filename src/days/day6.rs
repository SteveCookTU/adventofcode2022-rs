use std::collections::HashSet;

pub fn part1(input: &[String]) -> usize {
    let line = &input[0];
    let mut processed_characters = 0;
    for (i, window) in line.as_bytes().windows(4).enumerate() {
        if window[1..].iter().any(|i| i == &window[0]) {
            continue;
        }
        if window[2..].iter().any(|i| i == &window[1]) {
            continue;
        }
        if window[2] == window[3] {
            continue;
        }
        processed_characters = i + 4;
        break;
    }

    processed_characters
}

pub fn part2(input: &[String]) -> usize {
    let line = &input[0];
    let mut processed_characters = 0;
    let mut set = HashSet::with_capacity(14);
    'outer: for (i, window) in line.as_bytes().windows(14).enumerate() {
        for i in window {
            if !set.insert(*i) {
                set.clear();
                continue 'outer;
            }
        }
        processed_characters = i + 14;
        break;
    }

    processed_characters
}

#[cfg(test)]
mod tests {
    use crate::{day6, parse_input_static};

    #[test]
    fn test_part1() {
        let input = parse_input_static("nppdvjthqldpwncqszvftbrmjlhg");

        assert_eq!(6, day6::part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input_static("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

        assert_eq!(19, day6::part2(&input));
    }
}
