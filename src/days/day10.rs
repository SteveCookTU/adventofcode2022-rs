use std::str::FromStr;

pub fn part1(input: &[&str]) -> i32 {
    let mut register = 1;
    let mut cycle_counter = 0;

    let mut signal_strength_sum = 0;

    for line in input {
        if cycle_counter >= 220 {
            break;
        }
        if line.starts_with("noop") {
            cycle_counter += 1;
            if (cycle_counter - 20) % 40 == 0 {
                signal_strength_sum += register * cycle_counter;
            }
        } else if line.starts_with("addx") {
            cycle_counter += 1;
            if (cycle_counter - 20) % 40 == 0 {
                signal_strength_sum += register * cycle_counter;
            }
            cycle_counter += 1;
            if (cycle_counter - 20) % 40 == 0 {
                signal_strength_sum += register * cycle_counter;
            }
            let to_add = i32::from_str(line.split_once(' ').unwrap().1).unwrap();
            register += to_add;
        }
    }

    signal_strength_sum
}

pub fn part2(input: &[&str]) -> String {
    let mut register = 1;
    let mut cycle_counter = 0;

    let mut crt = vec!['.'; 240];

    for line in input {
        if line.starts_with("noop") {
            if (register - 1..=register + 1).contains(&(cycle_counter % 40)) {
                crt[cycle_counter as usize] = '#';
            }
            cycle_counter += 1;
        } else if line.starts_with("addx") {
            if (register - 1..=register + 1).contains(&(cycle_counter % 40)) {
                crt[cycle_counter as usize] = '#';
            }
            cycle_counter += 1;
            if (register - 1..=register + 1).contains(&(cycle_counter % 40)) {
                crt[cycle_counter as usize] = '#';
            }
            cycle_counter += 1;
            let to_add = i32::from_str(line.split_once(' ').unwrap().1).unwrap();
            register += to_add;
        }
    }
    crt.chunks(40)
        .map(|s| s.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}
