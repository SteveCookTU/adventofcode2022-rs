use std::str::FromStr;

pub fn part1(input: &[&str]) -> u32 {
    let mut max = 0;
    let mut current = 0;
    for line in input {
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            current += u32::from_str(line).unwrap();
        }
    }
    if current > max {
        max = current;
    }
    max
}

pub fn part2(input: &[&str]) -> u32 {
    let mut values = Vec::new();
    let mut current = 0;
    for line in input {
        if line.is_empty() {
            values.push(current);
            current = 0;
        } else {
            current += u32::from_str(line).unwrap();
        }
    }
    values.push(current);
    values.sort_by(|a, b| b.cmp(a));
    values.into_iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};
    use crate::parse_input_static;

    #[test]
    fn test_part1() {
        let input = parse_input_static(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );

        assert_eq!(24000, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input_static(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );

        assert_eq!(45000, part2(&input));
    }
}
