use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::{eyre::eyre, Result};
use once_cell::sync::Lazy;

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Result<CosmicMap> {
    Lazy::get(&COLOR_EYRE);
    parsers::parse_input(input).map_err(|e| eyre!(e.to_string()))
}

#[aoc(day11, part1)]
pub fn part1(input: &CosmicMap) -> Result<u64> {
    todo!()
}

#[aoc(day11, part2)]
pub fn part2(input: &CosmicMap) -> Result<i64> {
    todo!()
}

#[derive(Debug)]
enum CosmicEntry {
    Space,
    Galaxy(u64),
}

#[derive(Debug)]
pub struct CosmicMap {
    entries: Vec<Vec<CosmicEntry>>,
}

impl CosmicMap {
    fn expand(&mut self) {
        todo!()
    }

    fn is_empty_column(&self, col: usize) -> bool {
        self.entries
            .iter()
            .map(|row| row.get(col).expect("bounds are checked before usage"))
            .all(|entry| matches!(entry, CosmicEntry::Space))
    }

    fn is_empty_row(&self, row: usize) -> bool {
        self.entries
            .get(row)
            .expect("bounds are checked before usage")
            .iter()
            .all(|entry| matches!(entry, CosmicEntry::Space))
    }
}

mod parsers {
    use nom::{
        character::complete::{line_ending, one_of},
        multi::{many1, separated_list1},
        IResult, Parser,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{error::ErrorTree, final_parser::final_parser, ParserExt};

    use crate::day11::{CosmicEntry, CosmicMap};

    type ParseError<'a> = ErrorTree<Span<'a>>;

    type Span<'a> = LocatedSpan<&'a str>;

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<CosmicMap, ParseError> {
        final_parser(cosmic_map)(Span::new(input))
    }

    fn cosmic_map(input: Span) -> IResult<Span, CosmicMap, ParseError> {
        let mut next_galaxy_id = 0;
        separated_list1(line_ending, map_line)
            .map(|rows| {
                rows.into_iter()
                    .map(move |entries| {
                        entries
                            .into_iter()
                            .map(|entry| {
                                if matches!(entry, CosmicEntry::Galaxy(_)) {
                                    let galaxy = CosmicEntry::Galaxy(next_galaxy_id);
                                    next_galaxy_id += 1;
                                    galaxy
                                } else {
                                    entry
                                }
                            })
                            .collect()
                    })
                    .collect()
            })
            .map(|entries| CosmicMap { entries })
            .parse(input)
    }

    fn map_line(input: Span) -> IResult<Span, Vec<CosmicEntry>, ParseError> {
        many1(cosmic_entry).parse(input)
    }

    fn cosmic_entry(input: Span) -> IResult<Span, CosmicEntry, ParseError> {
        one_of(".#")
            .context("valid object syntax")
            .map(|c| match c {
                '.' => CosmicEntry::Space,
                '#' => CosmicEntry::Galaxy(0),
                _ => unreachable!("invalid map object"),
            })
            .context("convert to valid CosmicEntry")
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE_INPUT: &str = indoc! {
        "...#......
         .......#..
         #.........
         ..........
         ......#...
         .#........
         .........#
         ..........
         .......#..
         #...#....."
    };

    #[test]
    fn test_parse_sample_input() -> Result<()> {
        let parsed_cosmic_map = input_generator(SAMPLE_INPUT)?;
        insta::assert_debug_snapshot!(parsed_cosmic_map);
        Ok(())
    }
}
