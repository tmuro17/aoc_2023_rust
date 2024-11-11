use crate::utils::point::Point;
use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::{eyre::eyre, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::fmt::{Display, Formatter, Write};

static COLOR_EYRE: Lazy<()> = Lazy::new(|| color_eyre::install().unwrap());

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Result<CosmicMap> {
    Lazy::get(&COLOR_EYRE);
    parsers::parse_input(input).map_err(|e| eyre!(e.to_string()))
}

#[aoc(day11, part1)]
pub fn part1(input: &CosmicMap) -> Result<u64> {
    let mut input = input.clone();
    input.expand();
    let poses = input.galaxy_positions();
    let pairs = generate_pairs(poses);

    Ok(pairs.into_iter().map(|pair| pair_distance(pair)).sum())
}

#[aoc(day11, part2)]
pub fn part2(input: &CosmicMap) -> Result<i64> {
    todo!()
}

#[derive(Debug, Clone)]
enum CosmicEntry {
    Space,
    Galaxy(u64),
}

#[derive(Debug, Clone)]
pub struct CosmicMap {
    entries: Vec<Vec<CosmicEntry>>,
}

#[derive(Debug, Clone)]
pub struct GalaxyPosition {
    id: u64,
    position: Point,
}

impl Display for CosmicMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.entries {
            let row = row.iter().join("");
            writeln!(f, "{}", row)?;
        }

        Ok(())
    }
}

impl Display for CosmicEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            CosmicEntry::Space => '.',
            CosmicEntry::Galaxy(_) => '#',
        })
    }
}

impl CosmicMap {
    fn expand(&mut self) {
        let empty_rows: Vec<_> = (0..self.entries.len())
            .filter(|row| self.is_empty_row(*row))
            .collect();

        let number_cols = self.entries.first().unwrap().len();
        let empty_cols: Vec<_> = (0..number_cols)
            .filter(|col| self.is_empty_column(*col))
            .collect();

        empty_rows.iter().fold(0, |acc, row| {
            self.insert_row(row + acc);
            acc + 1
        });
        empty_cols.iter().fold(0, |acc, col| {
            self.insert_column(col + acc);
            acc + 1
        });
    }

    fn galaxy_positions(&self) -> Vec<GalaxyPosition> {
        let mut positions = vec![];
        for (y, row) in self.entries.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if let CosmicEntry::Galaxy(id) = col {
                    positions.push(GalaxyPosition {
                        id: *id,
                        position: Point::from((x as i64, y as i64)),
                    });
                }
            }
        }
        positions
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

    fn insert_row(&mut self, after_row: usize) {
        let col_count = self.entries.first().unwrap().len();
        let empty = vec![CosmicEntry::Space; col_count];
        self.entries.insert(after_row + 1, empty);
    }

    fn insert_column(&mut self, after_col: usize) {
        for row in self.entries.iter_mut() {
            row.insert(after_col + 1, CosmicEntry::Space);
        }
    }
}

fn generate_pairs(galaxies: Vec<GalaxyPosition>) -> Vec<(GalaxyPosition, GalaxyPosition)> {
    galaxies
        .into_iter()
        .combinations(2)
        .map(|arr| (arr.first().unwrap().clone(), arr.get(1).unwrap().clone()))
        .collect()
}

fn pair_distance(pair: (GalaxyPosition, GalaxyPosition)) -> u64 {
    let (left, right) = pair;

    left.position.manhattan_distance(&right.position)
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
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

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

    #[test]
    fn test_map_expansion() -> Result<()> {
        let expected_map = indoc! {
            "....#........
             .........#...
             #............
             .............
             .............
             ........#....
             .#...........
             ............#
             .............
             .............
             .........#...
             #....#......."
        };
        let expected = input_generator(expected_map)?;
        let mut input = input_generator(SAMPLE_INPUT)?;

        input.expand();

        assert_eq!(format!("{input}"), format!("{expected}"));

        Ok(())
    }

    #[test]
    fn test_galaxy_positions() -> Result<()> {
        let parsed_cosmic_map = input_generator(SAMPLE_INPUT)?;
        let positions = parsed_cosmic_map.galaxy_positions();
        insta::assert_debug_snapshot!(positions);
        Ok(())
    }

    #[test]
    fn test_pairs() -> Result<()> {
        let parsed_cosmic_map = input_generator(SAMPLE_INPUT)?;
        let positions = parsed_cosmic_map.galaxy_positions();
        let pairs = generate_pairs(positions);
        assert_eq!(pairs.len(), 36);
        Ok(())
    }

    #[test]
    fn part1_sample_input() -> Result<()> {
        let parsed_cosmic_map = input_generator(SAMPLE_INPUT)?;
        let res = part1(&parsed_cosmic_map).unwrap();
        assert_eq!(res, 374);
        Ok(())
    }
}
