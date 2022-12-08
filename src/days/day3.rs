pub fn part1(input: &[String]) -> u32 {
    let mut priority_sum = 0;
    for line in input {
        let full_bytes = line.as_bytes();
        let len = full_bytes.len() / 2;
        let first = &full_bytes[..len];
        let second = &full_bytes[len..];

        let mut in_both = *first.iter().find(|i| second.contains(i)).unwrap();

        if in_both < 97 {
            in_both -= 38;
        } else {
            in_both -= 96;
        }
        priority_sum += in_both as u32;
    }
    priority_sum
}

pub fn part2(input: &[String]) -> u32 {
    let mut priority_sum = 0;
    for line in input.chunks_exact(3) {
        let first = line[0].as_bytes();
        let second = line[1].as_bytes();
        let third = line[2].as_bytes();

        let mut in_both = *first
            .iter()
            .find(|i| second.contains(i) && third.contains(i))
            .unwrap();

        if in_both < 97 {
            in_both -= 38;
        } else {
            in_both -= 96;
        }
        priority_sum += in_both as u32;
    }
    priority_sum
}

#[cfg(test)]
mod tests {
    use crate::{day3, parse_input_static};

    #[test]
    fn test_part1() {
        let input = parse_input_static(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        assert_eq!(157, day3::part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input_static(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        assert_eq!(70, day3::part2(&input));
    }
}
