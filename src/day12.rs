use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::Result;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::fmt::{Display, Write};

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Result<String> {
    Lazy::get(&COLOR_EYRE);
    todo!()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
struct SpringGroup {
    state: SpringState,
    count: usize,
}

type CheckSum = Vec<u64>;

#[derive(Debug, Clone)]
struct SpringRecord {
    groups: Vec<SpringGroup>,
    check_sum: CheckSum,
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> Result<u64> {
    todo!()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> Result<u64> {
    todo!()
}

mod parsers {
    use crate::day12::{CheckSum, SpringGroup, SpringRecord, SpringState};
    use itertools::Itertools;
    use nom::{
        character::complete::{line_ending, one_of, space1, u64},
        multi::{many1, separated_list1},
        IResult, Parser,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{
        error::ErrorTree, final_parser::final_parser, tag::complete::tag, ParserExt,
    };

    type ParseError<'a> = ErrorTree<Span<'a>>;

    type Span<'a> = LocatedSpan<&'a str>;

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<Vec<SpringRecord>, ParseError> {
        final_parser(spring_records)(Span::new(input))
    }

    fn spring_records(input: Span) -> IResult<Span, Vec<SpringRecord>, ParseError> {
        separated_list1(line_ending, spring_record).parse(input)
    }

    fn spring_record(input: Span) -> IResult<Span, SpringRecord, ParseError> {
        many1(spring_state)
            .and(check_sum.preceded_by(space1))
            .map(|(springs, sum)| {
                let groups = springs
                    .into_iter()
                    .group_by(|x| *x)
                    .into_iter()
                    .map(|(state, g)| SpringGroup {
                        state,
                        count: g.count(),
                    })
                    .collect();

                SpringRecord {
                    groups,
                    check_sum: sum,
                }
            })
            .parse(input)
    }

    fn check_sum(input: Span) -> IResult<Span, CheckSum, ParseError> {
        separated_list1(tag(","), u64).parse(input)
    }

    fn spring_state(input: Span) -> IResult<Span, SpringState, ParseError> {
        one_of(".#?")
            .context("valid spring syntax")
            .map(|c| match c {
                '.' => SpringState::Operational,
                '#' => SpringState::Damaged,
                '?' => SpringState::Unknown,
                _ => unreachable!("invalid spring state"),
            })
            .context("convert to valid SpringState")
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::parsers::parse_input;
    use color_eyre::eyre::Result;
    use indoc::indoc;
    use insta::assert_debug_snapshot;
    const SAMPLE_INPUT: &str = indoc! {
        "???.### 1,1,3
         .??..??...?##. 1,1,3
         ?#?#?#?#?#?#?#? 1,3,1,6
         ????.#...#... 4,1,1
         ????.######..#####. 1,6,5
         ?###???????? 3,2,1"
    };

    #[test]
    fn test_parser() -> Result<()> {
        let actual = parse_input(SAMPLE_INPUT)?;
        assert_debug_snapshot!(actual);
        Ok(())
    }
}
