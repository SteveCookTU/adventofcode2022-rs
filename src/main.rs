use adventofcode2022::{
    day1, day10, day11, day12, day2, day3, day4, day5, day6, day7, day8, day9, format_input,
    get_input,
};
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut input_str = get_input("./inputs/day1.txt")?;

    {
        let input = format_input(&input_str);
        let day1_part1 = day1::part1(&input);
        let day1_part2 = day1::part2(&input);
        println!("Day 1: Max calories is {day1_part1}");
        println!("Day 1: Sum of top 3 calories is {day1_part2}");
    }

    input_str = get_input("./inputs/day2.txt")?;
    {
        let input = format_input(&input_str);
        let day2_part1 = day2::part1(&input);
        let day2_part2 = day2::part2(&input);
        println!("Day 2: Total score is {day2_part1}");
        println!("Day 2: Total score is {day2_part2}");
    }

    input_str = get_input("./inputs/day3.txt")?;
    {
        let input = format_input(&input_str);
        let day3_part1 = day3::part1(&input);
        let day3_part2 = day3::part2(&input);
        println!("Day 3: Priority Sum is {day3_part1}");
        println!("Day 3: Group Priority Sum is {day3_part2}");
    }

    input_str = get_input("./inputs/day4.txt")?;
    {
        let input = format_input(&input_str);
        let day4_part1 = day4::part1(&input);
        let day4_part2 = day4::part2(&input);
        println!("Day 4: {day4_part1} fully containing assignments");
        println!("Day 4: {day4_part2} partially overlapping assignments");
    }

    input_str = get_input("./inputs/day5.txt")?;
    {
        let input = format_input(&input_str);
        let day5_part1 = day5::part1(&input);
        let day5_part2 = day5::part2(&input);
        println!("Day 5: The tops of the stacks are {day5_part1}");
        println!("Day 5: The tops of the stacks are {day5_part2}");
    }

    input_str = get_input("./inputs/day6.txt")?;
    {
        let input = format_input(&input_str);
        let day6_part1 = day6::part1(&input);
        let day6_part2 = day6::part2(&input);
        println!("Day 6: The first start marker ends at index {day6_part1}");
        println!("Day 6: The first message marker ends at index {day6_part2}");
    }

    input_str = get_input("./inputs/day7.txt")?;
    {
        let input = format_input(&input_str);
        let day7_part1 = day7::part1(&input);
        let day7_part2 = day7::part2(&input);
        println!("Day 7: The sum of small directories is {day7_part1}");
        println!("Day 7: Smallest directory to delete is {day7_part2} bytes");
    }

    input_str = get_input("./inputs/day8.txt")?;
    {
        let input = format_input(&input_str);
        let day8_part1 = day8::part1(&input);
        let day8_part2 = day8::part2(&input);
        println!("Day 8: {day8_part1} visible trees");
        println!("Day 8: Max view distance is {day8_part2}");
    }

    input_str = get_input("./inputs/day9.txt")?;
    {
        let input = format_input(&input_str);
        let day9_part1 = day9::part1(&input);
        let day9_part2 = day9::part2(&input);
        println!("Day 9: {day9_part1} unique positions");
        println!("Day 9: {day9_part2} unique positions");
    }

    input_str = get_input("./inputs/day10.txt")?;
    {
        let input = format_input(&input_str);
        let day10_part1 = day10::part1(&input);
        let day10_part2 = day10::part2(&input);
        println!("Day 10: Signal strength sum is {day10_part1}");
        println!("Day 10: Output is\n{day10_part2}");
    }

    input_str = get_input("./inputs/day11.txt")?;
    {
        let input = format_input(&input_str);
        let day11_part1 = day11::part1(&input);
        let day11_part2 = day11::part2(&input);
        println!("Day 11: Level of monkey business is {day11_part1}");
        println!("Day 11: Level of monkey business is {day11_part2}");
    }

    input_str = get_input("./inputs/day12.txt")?;
    {
        let input = format_input(&input_str);
        let day12_part1 = day12::part1(&input);
        let day12_part2 = day12::part2(&input);
        println!("Day 12: The shortest path is {day12_part1} steps");
        println!("Day 12: The shortest path is {day12_part2} steps");
    }
    Ok(())
}
