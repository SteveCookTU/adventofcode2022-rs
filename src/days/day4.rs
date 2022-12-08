use std::str::FromStr;

pub fn part1(input: &[String]) -> u32 {
    let mut count = 0;

    for line in input {
        let (first, second) = line.split_once(',').unwrap();
        let (first_min, first_max) = first.split_once('-').unwrap();
        let (second_min, second_max) = second.split_once('-').unwrap();
        let (first_min, first_max) = (
            u32::from_str(first_min).unwrap(),
            u32::from_str(first_max).unwrap(),
        );
        let (second_min, second_max) = (
            u32::from_str(second_min).unwrap(),
            u32::from_str(second_max).unwrap(),
        );

        if (first_min >= second_min && first_max <= second_max)
            || (second_min >= first_min && second_max <= first_max)
        {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &[String]) -> u32 {
    let mut count = 0;

    for line in input {
        let (first, second) = line.split_once(',').unwrap();
        let (first_min, first_max) = first.split_once('-').unwrap();
        let (second_min, second_max) = second.split_once('-').unwrap();
        let (first_min, first_max) = (
            u32::from_str(first_min).unwrap(),
            u32::from_str(first_max).unwrap(),
        );
        let (second_min, second_max) = (
            u32::from_str(second_min).unwrap(),
            u32::from_str(second_max).unwrap(),
        );

        if first_min <= second_max && first_max >= second_min {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::{day4, parse_input_static};

    #[test]
    pub fn test_part1() {
        let input = parse_input_static(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );

        assert_eq!(2, day4::part1(&input));
    }

    #[test]
    pub fn test_part2() {
        let input = parse_input_static(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        );

        assert_eq!(4, day4::part2(&input));
    }
}
