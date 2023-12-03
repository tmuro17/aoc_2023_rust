use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1, part1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(str::to_owned).collect()
}

#[aoc_generator(day1, part2)]
pub fn input_generator_part_2(input: &str) -> Vec<String> {
    let patterns = vec![
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    input
        .lines()
        .map(|line| {
            let mut result = String::new();
            for x in 0..line.len() {
                for (pattern, digit) in &patterns {
                    if line[x..].starts_with(pattern) {
                        result.push_str(digit);
                    }
                }
            }
            result
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(lines: &[String]) -> u64 {
    lines
        .iter()
        .map(|x| {
            let digits: Vec<_> = x.chars().filter(char::is_ascii_digit).collect();

            let first = digits.first().expect("there is a first digit");
            let last = digits.last().expect("there is a last digit");

            let combined = {
                let mut x = first.to_string();
                x.push(*last);
                x
            };

            combined.parse::<u64>().expect("valid number")
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(lines: &[String]) -> u64 {
    part1(lines)
}
