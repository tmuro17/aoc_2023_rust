use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::{Report, Result};
use itertools::Itertools;
use nom::{combinator::all_consuming, Finish};

use crate::day9::parsers::scan_line;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Result<Scan> {
    parsers::parse_input(input)
}

#[aoc(day9, part1)]
pub fn part1(input: &Scan) -> Result<i64> {
    Ok(input.lines.iter().map(|line| line.next_value()).sum())
}

#[aoc(day9, part2)]
pub fn part2(input: &Scan) -> Result<i64> {
    Ok(input.lines.iter().map(|line| line.first_value()).sum())
}

#[derive(Debug, Clone)]
pub struct Scan {
    lines: Vec<ScanLine>,
}

#[derive(Debug, Clone)]
struct ScanLine {
    values: Vec<i64>,
}

impl ScanLine {
    fn sequences(&self) -> Vec<Vec<i64>> {
        let mut current = self.values.clone();
        let mut sequences = vec![];

        while !current.iter().all(|n| n == &0) {
            let new_current = current
                .iter()
                .tuple_windows()
                .fold(Vec::new(), |mut acc, (x, y)| {
                    acc.push(y - x);
                    acc
                });
            sequences.push(current);
            current = new_current;
        }
        sequences.push(current);

        sequences
    }

    fn next_value(&self) -> i64 {
        let mut sequences = self.sequences();

        sequences
            .iter_mut()
            .rev()
            .fold(None, |prev: Option<&mut Vec<i64>>, seq| {
                if let Some(prev) = prev {
                    let last_prev = prev.last().unwrap();
                    let last_seq = seq.last().unwrap();

                    seq.push(last_seq + last_prev);
                } else {
                    seq.push(0);
                }

                Some(seq)
            });

        *sequences.first().unwrap().last().unwrap()
    }

    fn first_value(&self) -> i64 {
        let mut sequences = self.sequences();

        sequences
            .iter_mut()
            .rev()
            .fold(None, |prev: Option<&mut Vec<i64>>, seq| {
                if let Some(prev) = prev {
                    let first_prev = prev.first().unwrap();
                    let first_seq = seq.first().unwrap();

                    seq.insert(0, first_seq - first_prev);
                } else {
                    seq.insert(0, 0);
                }

                Some(seq)
            });

        *sequences.first().unwrap().first().unwrap()
    }
}

impl TryFrom<&str> for ScanLine {
    type Error = Report;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (_, scan_line) = all_consuming(scan_line)(value)
            .map_err(|e| e.to_owned())
            .finish()?;

        Ok(scan_line)
    }
}

mod parsers {
    use nom::{
        character,
        character::complete::{newline, space1},
        combinator::{all_consuming, map},
        multi::separated_list1,
        Finish, IResult,
    };

    use crate::day9::{Scan, ScanLine};

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<Scan> {
        let (_, scan) = all_consuming(scan)(input)
            .map_err(|e| e.to_owned())
            .finish()?;
        Ok(scan)
    }

    fn scan(input: &str) -> IResult<&str, Scan> {
        map(separated_list1(newline, scan_line), |lines| Scan { lines })(input)
    }

    pub(crate) fn scan_line(input: &str) -> IResult<&str, ScanLine> {
        map(
            separated_list1(space1, character::complete::i64),
            |values| ScanLine { values },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;
    use indoc::indoc;
    use rstest::rstest;

    use super::*;

    const SAMPLE_INPUT: &str = indoc! {
     "0 3 6 9 12 15
      1 3 6 10 15 21
      10 13 16 21 30 45"   
    };

    #[test]
    fn parse_sample_input() -> Result<()> {
        let parsed_scan = input_generator(SAMPLE_INPUT)?;
        insta::assert_debug_snapshot!(parsed_scan);
        Ok(())
    }

    #[test]
    fn part1_sample_input() -> Result<()> {
        let scan = input_generator(SAMPLE_INPUT)?;
        let actual = part1(&scan)?;
        assert_eq!(114, actual);
        Ok(())
    }

    #[rstest]
    #[case::first_history("0 3 6 9 12 15")]
    #[case::second_history("1 3 6 10 15 21")]
    #[case::third_history("10 13 16 21  30 45")]
    fn sequences_from_examples(#[case] example_history: &str) -> Result<()> {
        let scan_line: ScanLine = example_history.try_into()?;
        let sequences = scan_line.sequences();
        insta::assert_debug_snapshot!(example_history, sequences);
        Ok(())
    }

    #[rstest]
    #[case::first_history("0 3 6 9 12 15", 18)]
    #[case::second_history("1 3 6 10 15 21", 28)]
    #[case::third_history("10 13 16 21  30 45", 68)]
    fn next_history_from_examples(
        #[case] example_history: &str,
        #[case] expected: i64,
    ) -> Result<()> {
        let scan_line: ScanLine = example_history.try_into()?;
        let next_history = scan_line.next_value();

        assert_eq!(expected, next_history);
        Ok(())
    }

    #[rstest]
    #[case::first_history("0 3 6 9 12 15", -3)]
    #[case::second_history("1 3 6 10 15 21", 0)]
    #[case::third_history("10 13 16 21  30 45", 5)]
    fn first_history_from_examples(
        #[case] example_history: &str,
        #[case] expected: i64,
    ) -> Result<()> {
        let scan_line: ScanLine = example_history.try_into()?;
        let next_history = scan_line.first_value();

        assert_eq!(expected, next_history);
        Ok(())
    }

    #[test]
    fn part2_sample_input() -> Result<()> {
        let scan = input_generator(SAMPLE_INPUT)?;
        let actual = part2(&scan)?;
        assert_eq!(2, actual);
        Ok(())
    }
}
