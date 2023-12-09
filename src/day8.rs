use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::{eyre::eyre, Result};
use num::integer::lcm;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Result<Map> {
    parsers::parse_input(input)
}

#[aoc(day8, part1)]
pub fn part1(map: &Map) -> Result<usize> {
    let mut steps = map.steps.iter().cycle();
    let mut current = map.nodes.get("AAA").unwrap();
    let mut count = 0;

    while current.id != "ZZZ" {
        let step = steps.next().unwrap();

        current = match step {
            Step::Left => map.nodes.get(&current.left).unwrap(),
            Step::Right => map.nodes.get(&current.right).unwrap(),
        };

        count += 1;
    }

    Ok(count)
}

#[aoc(day8, part2)]
pub fn part2(map: &Map) -> Result<usize> {
    let steps = map.steps.iter().cycle();
    let current: Vec<_> = map
        .nodes
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, v)| v)
        .collect();

    // Had to find the LCM method on Reddit,
    // I knew that the iterative solution wouldn't work,
    // but I never seem to realize the LCM optimizations that are frequently possible for AOC

    let path_step = current
        .iter()
        .map(|&cur| {
            let mut inner_current = cur;
            let mut steps = steps.clone();
            let mut count = 0_usize;

            while !inner_current.id.ends_with('Z') {
                let step = steps.next().unwrap();

                inner_current = match step {
                    Step::Left => map.nodes.get(&inner_current.left).unwrap(),
                    Step::Right => map.nodes.get(&inner_current.right).unwrap(),
                };

                count += 1;
            }
            count
        })
        .fold(None, |acc, num| {
            if let Some(acc) = acc {
                Some(lcm(acc, num))
            } else {
                Some(num)
            }
        });

    path_step.ok_or(eyre!("no valid items"))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Step {
    Left,
    Right,
}

type Steps = Vec<Step>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    id: String,
    left: String,
    right: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map {
    steps: Steps,
    nodes: HashMap<String, Node>,
}

mod parsers {
    use std::collections::HashMap;

    use color_eyre::eyre::eyre;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alphanumeric1, newline, space1},
        combinator,
        combinator::all_consuming,
        multi::{many1, separated_list1},
        sequence::{delimited, preceded, separated_pair, terminated},
        Finish, IResult,
    };

    use crate::day8::{Map, Node, Step, Steps};

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<Map> {
        let (_, map) = all_consuming(map)(input)
            .map_err(|e| e.to_owned())
            .finish()?;
        Ok(map)
    }

    fn map(input: &str) -> IResult<&str, Map> {
        combinator::map(
            separated_pair(terminated(steps, newline), newline, nodes),
            |(steps, nodes)| {
                let node_map: HashMap<String, Node> = nodes
                    .into_iter()
                    .map(|node| (node.id.clone(), node))
                    .collect();

                Map {
                    steps,
                    nodes: node_map,
                }
            },
        )(input)
    }

    fn steps(input: &str) -> IResult<&str, Steps> {
        many1(step)(input)
    }

    fn step(input: &str) -> IResult<&str, Step> {
        combinator::map_res(alt((tag("L"), tag("R"))), |step| match step {
            "L" => Ok(Step::Left),
            "R" => Ok(Step::Right),
            _ => Err(eyre!("invalid step")),
        })(input)
    }

    fn node(input: &str) -> IResult<&str, Node> {
        combinator::map(
            separated_pair(
                alphanumeric1,
                preceded(space1, terminated(tag("="), space1)),
                node_pair,
            ),
            |(id, (l_id, r_id))| Node {
                id: id.to_owned(),
                left: l_id.to_owned(),
                right: r_id.to_owned(),
            },
        )(input)
    }

    fn node_pair(input: &str) -> IResult<&str, (&str, &str)> {
        delimited(
            tag("("),
            separated_pair(alphanumeric1, terminated(tag(","), space1), alphanumeric1),
            tag(")"),
        )(input)
    }

    fn nodes(input: &str) -> IResult<&str, Vec<Node>> {
        separated_list1(newline, node)(input)
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;
    use itertools::Itertools;

    use super::*;

    const SAMPLE_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_parser() -> Result<()> {
        let map = input_generator(SAMPLE_INPUT)?;
        assert_eq!(vec![Step::Right, Step::Left], map.steps);
        let nodes: Vec<_> = map
            .nodes
            .into_iter()
            .sorted_by_key(|(k, _)| k.clone())
            .collect();
        insta::assert_debug_snapshot!(nodes);
        Ok(())
    }

    #[test]
    fn part1_sample_input() -> Result<()> {
        let map = input_generator(SAMPLE_INPUT)?;
        let actual = part1(&map)?;

        assert_eq!(2, actual);
        Ok(())
    }
    #[test]
    fn part1_sample_input_case_2() -> Result<()> {
        let sample_2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let map = input_generator(sample_2)?;
        let actual = part1(&map)?;

        assert_eq!(6, actual);
        Ok(())
    }

    #[test]
    fn part2_example_input() -> Result<()> {
        let sample = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let map = input_generator(sample)?;
        let actual = part2(&map)?;

        assert_eq!(6, actual);
        Ok(())
    }
}
