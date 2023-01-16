use std::collections::VecDeque;
use std::str::FromStr;

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
}

impl Monkey {
    fn test(&self, num: u64) -> bool {
        num % self.test == 0
    }
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            items: VecDeque::new(),
            operation: Box::new(|i| i),
            test: 0,
            if_true: 0,
            if_false: 0,
            inspections: 0,
        }
    }
}

fn parse_monkeys(input: &[&str]) -> Vec<Monkey> {
    let mut curr_monkey = Monkey::default();
    let mut monkeys = Vec::new();
    for mut line in input.iter().copied() {
        if line.is_empty() {
            monkeys.push(curr_monkey);
            curr_monkey = Monkey::default();
        }
        if line.contains("Monkey") {
            continue;
        }
        line = line.trim();
        if line.starts_with("Starting") {
            curr_monkey.items = line
                .trim_start_matches("Starting items: ")
                .split(", ")
                .map(|s| u64::from_str(s).unwrap())
                .collect();
            continue;
        }
        if line.starts_with("Operation") {
            line = line.trim_start_matches("Operation: new = old ");
            let mut params = line.split(' ');
            let operation = params.next().unwrap();
            let num = params.next().unwrap();
            curr_monkey.operation = if let Ok(num) = u64::from_str(num) {
                match operation {
                    "*" => Box::new(move |i| i * num),
                    "+" => Box::new(move |i| i + num),
                    _ => Box::new(|i| i),
                }
            } else {
                match operation {
                    "*" => Box::new(move |i| i * i),
                    "+" => Box::new(move |i| i + i),
                    _ => Box::new(|i| i),
                }
            };
            continue;
        }
        if line.starts_with("Test") {
            curr_monkey.test =
                u64::from_str(line.trim_start_matches("Test: divisible by ")).unwrap();
            continue;
        }
        if line.starts_with("If true") {
            curr_monkey.if_true =
                usize::from_str(line.trim_start_matches("If true: throw to monkey ")).unwrap();
            continue;
        }
        if line.starts_with("If false") {
            curr_monkey.if_false =
                usize::from_str(line.trim_start_matches("If false: throw to monkey ")).unwrap();
            continue;
        }
    }
    monkeys.push(curr_monkey);
    monkeys
}

pub fn part1(input: &[&str]) -> u64 {
    let mut monkeys = parse_monkeys(input);

    (0..20).for_each(|_| {
        (0..monkeys.len()).for_each(|i| {
            monkeys[i].inspections += monkeys[i].items.len() as u64;
            while let Some(mut item) = monkeys[i].items.pop_front() {
                item = (monkeys[i].operation)(item) / 3;
                let index = if monkeys[i].test(item) {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[index].items.push_back(item);
            }
        });
    });

    monkeys.sort_by(|m2, m1| m1.inspections.cmp(&m2.inspections));
    monkeys[0].inspections * monkeys[1].inspections
}

pub fn part2(input: &[&str]) -> u64 {
    let mut monkeys = parse_monkeys(input);

    let common_multiple = monkeys.iter().map(|m| m.test).product::<u64>();

    (0..10000).for_each(|_| {
        (0..monkeys.len()).for_each(|i| {
            monkeys[i].inspections += monkeys[i].items.len() as u64;
            while let Some(mut item) = monkeys[i].items.pop_front() {
                item = (monkeys[i].operation)(item) % common_multiple;
                let index = if monkeys[i].test(item) {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[index].items.push_back(item);
            }
        });
    });

    monkeys.sort_by(|m2, m1| m1.inspections.cmp(&m2.inspections));
    monkeys[0].inspections * monkeys[1].inspections
}
