use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::Result;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn chances(&self) -> usize {
        (0..=self.time)
            .map(|hold| {
                let run = self.time - hold;

                hold * run
            })
            .filter(|&distance| distance > self.distance)
            .count()
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Result<Vec<Race>> {
    parsers::parse_input(input)
}

#[aoc(day6, part1)]
pub fn part1(races: &[Race]) -> usize {
    races.iter().map(Race::chances).product()
}

#[aoc(day6, part2)]
pub fn part2(races: &[Race]) -> Result<usize> {
    let (time, dist) = races.iter().fold(
        (String::new(), String::new()),
        |(mut time, mut dist), race| {
            time.push_str(race.time.to_string().as_str());
            dist.push_str(race.distance.to_string().as_str());

            (time, dist)
        },
    );

    let race = Race {
        time: time.parse()?,
        distance: dist.parse()?,
    };

    Ok(race.chances())
}

mod parsers {
    use color_eyre::Result;
    use nom::{
        bytes::complete::tag,
        character,
        character::complete::{newline, space1},
        combinator::{all_consuming, map},
        multi::separated_list1,
        sequence::{preceded, separated_pair, terminated},
        Finish, IResult,
    };

    use crate::day6::Race;

    pub(crate) fn parse_input(input: &str) -> Result<Vec<Race>> {
        let (_, races) = all_consuming(race_details)(input)
            .map_err(|e| e.to_owned())
            .finish()?;
        Ok(races)
    }

    fn race_details(input: &str) -> IResult<&str, Vec<Race>> {
        map(
            separated_pair(time_line, newline, distance_line),
            |(times, distances)| {
                times
                    .iter()
                    .zip(distances.iter())
                    .map(|(&time, &distance)| Race { time, distance })
                    .collect()
            },
        )(input)
    }

    fn time_line(input: &str) -> IResult<&str, Vec<u64>> {
        preceded(
            terminated(tag("Time:"), space1),
            separated_list1(space1, character::complete::u64),
        )(input)
    }

    fn distance_line(input: &str) -> IResult<&str, Vec<u64>> {
        preceded(
            terminated(tag("Distance:"), space1),
            separated_list1(space1, character::complete::u64),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;

    use crate::day6::parsers::parse_input;

    use super::*;

    static SAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn parser() -> Result<()> {
        let expected = vec![
            Race {
                time: 7,
                distance: 9,
            },
            Race {
                time: 15,
                distance: 40,
            },
            Race {
                time: 30,
                distance: 200,
            },
        ];

        let parsed = parse_input(SAMPLE_INPUT)?;

        assert_eq!(expected, parsed);

        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        let input = parse_input(SAMPLE_INPUT)?;

        let actual = part1(&input);

        assert_eq!(288, actual);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let input = parse_input(SAMPLE_INPUT)?;

        let actual = part2(&input)?;

        assert_eq!(71503, actual);

        Ok(())
    }
}
