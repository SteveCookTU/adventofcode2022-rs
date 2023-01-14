use std::str::FromStr;

pub fn part1(input: &[&str]) -> String {
    let mut inputs = input.split(|s| s.is_empty());
    let stack_input = inputs.next().unwrap();
    let stack_count = stack_input.last().unwrap().trim().replace(' ', "").len();
    let mut stacks = vec![vec![]; stack_count];

    let sanitized = &stack_input[..stack_input.len() - 1];
    for line in sanitized.iter().rev() {
        let crates = line.as_bytes().chunks(4);
        for (i, c) in crates.enumerate() {
            if c[1] == b' ' {
                continue;
            }
            stacks[i].push(c[1]);
        }
    }

    let instructions = inputs.next().unwrap();

    for instruction in instructions {
        let instruction = &instruction[5..];
        let (amount_str, move_to) = instruction.split_once(" from ").unwrap();
        let amount = usize::from_str(amount_str).unwrap();
        let (from_str, to_str) = move_to.split_once(" to ").unwrap();
        let from = usize::from_str(from_str).unwrap();
        let to = usize::from_str(to_str).unwrap();

        for _ in 0..amount {
            let to_move = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(to_move);
        }
    }

    let mut final_tops = String::new();

    stacks
        .iter()
        .for_each(|s| final_tops = format!("{}{}", final_tops, char::from(*s.last().unwrap())));

    final_tops
}

pub fn part2(input: &[&str]) -> String {
    let mut inputs = input.split(|s| s.is_empty());
    let stack_input = inputs.next().unwrap();
    let stack_count = stack_input.last().unwrap().trim().replace(' ', "").len();
    let mut stacks = vec![vec![]; stack_count];

    let sanitized = &stack_input[..stack_input.len() - 1];
    for line in sanitized.iter().rev() {
        let crates = line.as_bytes().chunks(4);
        for (i, c) in crates.enumerate() {
            if c[1] == b' ' {
                continue;
            }
            stacks[i].push(c[1]);
        }
    }

    let instructions = inputs.next().unwrap();

    for instruction in instructions {
        let instruction = &instruction[5..];
        let (amount_str, move_to) = instruction.split_once(" from ").unwrap();
        let amount = usize::from_str(amount_str).unwrap();
        let (from_str, to_str) = move_to.split_once(" to ").unwrap();
        let from = usize::from_str(from_str).unwrap();
        let to = usize::from_str(to_str).unwrap();

        let mut to_push = Vec::with_capacity(amount);

        for _ in 0..amount {
            to_push.push(stacks[from - 1].pop().unwrap());
        }

        for to_push in to_push.into_iter().rev() {
            stacks[to - 1].push(to_push);
        }
    }

    let mut final_tops = String::new();

    stacks
        .iter()
        .for_each(|s| final_tops = format!("{}{}", final_tops, char::from(*s.last().unwrap())));

    final_tops
}

#[cfg(test)]
mod tests {
    use crate::{day5, parse_input_static};

    #[test]
    pub fn test_part1() {
        let input = parse_input_static(
            "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );

        assert_eq!("CMZ".to_string(), day5::part1(&input));
    }

    #[test]
    pub fn test_part2() {
        let input = parse_input_static(
            "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );

        assert_eq!("MCD".to_string(), day5::part2(&input));
    }
}
