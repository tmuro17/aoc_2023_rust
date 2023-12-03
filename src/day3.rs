use std::{collections::HashMap, default::Default, ops::Index};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::point::Point;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Schematic {
    Schematic::from(input)
}

#[derive(Debug, Clone)]
pub struct Schematic {
    grid: Vec<Vec<char>>,
    cursor: Point,
}

impl Index<&Point> for Schematic {
    type Output = char;

    fn index(&self, index: &Point) -> &Self::Output {
        &self.grid[index.y as usize][index.x as usize]
    }
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let grid = value.lines().map(|line| line.chars().collect()).collect();

        Self {
            grid,
            cursor: Point::default(),
        }
    }
}

#[aoc(day3, part1)]
pub fn part1(schematic: &Schematic) -> u64 {
    let parts = part_numbers(schematic);
    parts.iter().sum()
}

fn part_numbers(schematic: &Schematic) -> Vec<u64> {
    let mut schematic = schematic.clone();
    let mut valid_nums: Vec<u64> = Vec::new();
    let max = Point {
        x: schematic.grid.first().unwrap().len() as i64 - 1,
        y: schematic.grid.len() as i64 - 1,
    };

    let mut acc = String::new();
    let mut valid = false;

    while schematic.cursor.y <= max.y {
        if !schematic[&schematic.cursor].is_ascii_digit() && !acc.is_empty() && valid {
            valid_nums.push(acc.parse().unwrap());
            valid = false;
            acc.clear();
        }

        if schematic[&schematic.cursor].is_ascii_digit() {
            acc.push(schematic[&schematic.cursor]);

            let valid_neighbors: Vec<Point> = schematic
                .cursor
                .neighbors()
                .into_iter()
                .filter(|pt| pt.is_valid(&(0, 0).into(), &max))
                .collect();

            let is_part = valid_neighbors
                .iter()
                .map(|pt| schematic[pt])
                .any(|c| c != '.' && !c.is_whitespace() && !c.is_ascii_digit());

            valid |= is_part;
        } else {
            acc.clear();
            valid = false;
        }

        schematic.cursor.x += 1;

        if schematic.cursor.x > max.x {
            if !acc.is_empty() && valid {
                valid_nums.push(acc.parse().unwrap());
            }

            acc.clear();
            schematic.cursor = Point {
                x: 0,
                y: schematic.cursor.y + 1,
            }
        }
    }

    valid_nums
}

fn potential_gears(schematic: &Schematic) -> Vec<(u64, Vec<Point>)> {
    let mut schematic = schematic.clone();
    let mut gear_nums: Vec<(u64, Vec<Point>)> = Vec::new();
    let max = Point {
        x: schematic.grid.first().unwrap().len() as i64 - 1,
        y: schematic.grid.len() as i64 - 1,
    };

    let mut acc = String::new();
    let mut valid = false;
    let mut gear_hubs = Vec::new();

    while schematic.cursor.y <= max.y {
        if !schematic[&schematic.cursor].is_ascii_digit()
            && !acc.is_empty()
            && valid
            && !gear_hubs.is_empty()
        {
            gear_nums.push((
                acc.parse().unwrap(),
                gear_hubs.iter().copied().unique().collect(),
            ));
            valid = false;
            gear_hubs.clear();
            acc.clear();
        }

        if schematic[&schematic.cursor].is_ascii_digit() {
            acc.push(schematic[&schematic.cursor]);

            let valid_neighbors: Vec<Point> = schematic
                .cursor
                .neighbors()
                .into_iter()
                .filter(|pt| pt.is_valid(&(0, 0).into(), &max))
                .collect();

            let is_part = valid_neighbors
                .iter()
                .map(|pt| schematic[pt])
                .any(|c| c != '.' && !c.is_whitespace() && !c.is_ascii_digit());

            let hubs = valid_neighbors.iter().filter(|pt| schematic[pt] == '*');

            gear_hubs.extend(hubs);

            valid |= is_part;
        } else {
            acc.clear();
            gear_hubs.clear();
            valid = false;
        }

        schematic.cursor.x += 1;

        if schematic.cursor.x > max.x {
            if !acc.is_empty() && valid {
                gear_nums.push((acc.parse().unwrap(), gear_hubs.clone()));
            }

            gear_hubs.clear();
            acc.clear();
            schematic.cursor = Point {
                x: 0,
                y: schematic.cursor.y + 1,
            }
        }
    }

    gear_nums
}

#[aoc(day3, part2)]
pub fn part2(input: &Schematic) -> u64 {
    let potential_gears = potential_gears(input);

    let mut hub_mapping: HashMap<Point, Vec<u64>> = HashMap::new();

    potential_gears.iter().for_each(|(num, hubs)| {
        hubs.iter().for_each(|&pt| {
            hub_mapping.entry(pt).or_default().push(*num);
        })
    });

    hub_mapping
        .iter()
        .filter(|(_, val)| val.len() == 2)
        .map(|(_, nums)| nums[0] * nums[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let input = input_generator(input);
        let expected_parts = vec![467, 35, 633, 617, 592, 755, 664, 598];
        let parts = part_numbers(&input);
        assert_eq!(expected_parts, parts);

        let actual = part1(&input);
        assert_eq!(4361, actual);
    }
    #[test]
    fn test_part1_reddit() {
        // Test case taken from reddit for help identify edge case that I couldn't see
        let input = r#"12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56"#;

        let input = input_generator(input);
        let actual = part1(&input);

        assert_eq!(925, actual);
    }

    #[test]
    fn test_part2() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let input = input_generator(input);
        let actual = part2(&input);
        assert_eq!(467_835, actual);
    }
}
