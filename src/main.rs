use adventofcode2022::{day1, day2, day3, day4, day5, day6, day7, get_input};
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut input = get_input("./inputs/day1.txt")?;
    let day1_part1 = day1::part1(&input);
    let day1_part2 = day1::part2(&input);
    println!("Day 1: Max calories is {}", day1_part1);
    println!("Day 1: Sum of top 3 calories is {}", day1_part2);

    input = get_input("./inputs/day2.txt")?;
    let day2_part1 = day2::part1(&input);
    let day2_part2 = day2::part2(&input);
    println!("Day 2: Total score is {}", day2_part1);
    println!("Day 2: Total score is {}", day2_part2);

    input = get_input("./inputs/day3.txt")?;
    let day3_part1 = day3::part1(&input);
    let day3_part2 = day3::part2(&input);
    println!("Day 3: Priority Sum is {}", day3_part1);
    println!("Day 3: Group Priority Sum is {}", day3_part2);

    input = get_input("./inputs/day4.txt")?;
    let day4_part1 = day4::part1(&input);
    let day4_part2 = day4::part2(&input);
    println!("Day 4: {} fully containing assignments", day4_part1);
    println!("Day 4: {} partially overlapping assignments", day4_part2);

    input = get_input("./inputs/day5.txt")?;
    let day5_part1 = day5::part1(&input);
    let day5_part2 = day5::part2(&input);
    println!("Day 5: The tops of the stacks are {}", day5_part1);
    println!("Day 5: The tops of the stacks are {}", day5_part2);

    input = get_input("./inputs/day6.txt")?;
    let day6_part1 = day6::part1(&input);
    let day6_part2 = day6::part2(&input);
    println!("Day 6: The first start marker ends at index {}", day6_part1);
    println!(
        "Day 6: The first message marker ends at index {}",
        day6_part2
    );

    input = get_input("./inputs/day7.txt")?;
    let day7_part1 = day7::part1(&input);
    let day7_part2 = day7::part2(&input);
    println!("Day 7: The sum of small directories is {}", day7_part1);
    println!(
        "Day 7: Smallest directory to delete is {} bytes",
        day7_part2
    );
    Ok(())
}
