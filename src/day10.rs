use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Write},
};
use std::sync::Once;

use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::{
    eyre::{Context, eyre, OptionExt},
    Report, Result,
};

static COLOR_EYRE: Once = Once::new();

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Result<Grid> {
    COLOR_EYRE.call_once(|| {
        color_eyre::install().unwrap()
    });
    parsers::parse_input(input).map_err(|e| eyre!(e.to_string()))
}

#[aoc(day10, part1)]
pub fn part1(input: &Grid) -> Result<u64> {
    let main_loop = input.main_loop().context("there must be a main loop")?;
    let (&start_pos, &start_tile) = main_loop
        .tiles
        .iter()
        .find(|(_, tile)| tile.tile_type == TileType::Start)
        .ok_or_eyre("there must be a start")?;

    let mut distances: HashMap<Pos, u64> = HashMap::new();
    distances.insert(start_pos, 0);

    let mut flows_from_start = start_tile
        .neighbors()
        .into_iter()
        .filter_map(|pos| main_loop.tiles.get(&pos))
        .filter(|tile| start_tile.flows(tile));

    let mut seen: HashSet<Pos> = HashSet::new();
    seen.insert(start_pos);
    let mut next = flows_from_start.next();
    let mut distance = 1;
    while let Some(current) = next {
        distances.insert(current.pos, distance);
        seen.insert(current.pos);
        distance += 1;

        next = current
            .neighbors()
            .iter()
            .filter_map(|pos| main_loop.tiles.get(pos))
            .find(|tile| current.flows(tile) && !seen.contains(&tile.pos));
    }

    seen.clear();
    seen.insert(start_pos);
    next = flows_from_start.next();
    distance = 1;

    while let Some(current) = next {
        let known_distance = distances
            .get_mut(&current.pos)
            .ok_or_eyre("tile should have been seen in first loop")?;

        *known_distance = min(distance, *known_distance);

        seen.insert(current.pos);
        distance += 1;

        next = current
            .neighbors()
            .iter()
            .filter_map(|pos| main_loop.tiles.get(pos))
            .find(|tile| current.flows(tile) && !seen.contains(&tile.pos));
    }

    distances
        .values()
        .max()
        .copied()
        .ok_or_eyre("there must be a max")
}

#[aoc(day10, part2)]
pub fn part2(input: &Grid) -> Result<u64> {
    let mut main_loop = input.main_loop().context("there must be a main loop")?;
    let check_loop = main_loop.clone();
    let (_, start_tile) = main_loop
        .tiles
        .iter_mut()
        .find(|(_, tile)| tile.tile_type == TileType::Start)
        .ok_or_eyre("there must be a start")?;

    let connectors: Vec<(Direction, TileType)> = start_tile
        .neighbors()
        .into_iter()
        .filter_map(|pos| check_loop.tiles.get(&pos))
        .filter(|tile| start_tile.flows(tile))
        .map(|tile| {
            let offset = tile.pos - start_tile.pos;
            (Direction::try_from(offset).unwrap(), tile.tile_type)
        })
        .collect();

    let s_type = [
        TileType::SouthWest90,
        TileType::SouthEast90,
        TileType::Vertical,
        TileType::Horizontal,
        TileType::NorthEast90,
        TileType::NorthWest90,
    ]
        .into_iter()
        .filter(|possible| {
            let (dir, tile) = &connectors[0];
            possible.flows(dir, tile)
        })
        .find(|possible| {
            let (dir, tile) = &connectors[1];
            possible.flows(dir, tile)
        })
        .unwrap();

    start_tile.tile_type = s_type;

    let mut outside = true;
    let mut count = 0;
    (0..main_loop.rows).for_each(|row| {
        (0..main_loop.cols).for_each(|col| {
            let pos = Pos::new(col as i64, row as i64);
            let tile_type = main_loop.tiles.get(&pos).unwrap().tile_type;

            if matches!(
                tile_type,
                TileType::Vertical | TileType::SouthWest90 | TileType::SouthEast90
            ) {
                outside = !outside;
            }

            if !outside && tile_type == TileType::Ground {
                count += 1;
            }
        });
        outside = true;
    });

    Ok(count)
}

type Pos = glam::I64Vec2;

#[derive(Debug, Clone)]
pub struct Grid {
    tiles: HashMap<Pos, Tile>,
    rows: usize,
    cols: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let tile = self
                    .tiles
                    .get(&Pos::new(col as i64, row as i64))
                    .expect("all points exist on the grid")
                    .tile_type;
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    fn main_loop(&self) -> Result<Self> {
        let (&start_pos, &start_tile) = self
            .tiles
            .iter()
            .find(|(_, tile)| tile.tile_type == TileType::Start)
            .ok_or_eyre("grid does not have a start position")?;

        let mut main_loop: HashMap<Pos, Tile> = HashMap::new();
        main_loop.insert(start_pos, start_tile);

        let mut next = start_tile
            .neighbors()
            .iter()
            .filter_map(|pos| self.tiles.get(pos))
            .find(|tile| start_tile.flows(tile) && !main_loop.contains_key(&tile.pos));

        while let Some(current) = next {
            main_loop.insert(current.pos, *current);

            next = current
                .neighbors()
                .iter()
                .filter_map(|pos| self.tiles.get(pos))
                .find(|tile| current.flows(tile) && !main_loop.contains_key(&tile.pos));
        }

        (0..self.rows).for_each(|row| {
            (0..self.cols).for_each(|col| {
                let pos = Pos::new(col as i64, row as i64);
                let _ = main_loop.entry(pos).or_insert_with(|| Tile {
                    tile_type: TileType::Ground,
                    pos,
                });
            })
        });

        Ok(Grid {
            tiles: main_loop,
            ..*self
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    tile_type: TileType,
    pos: Pos,
}

impl Tile {
    fn neighbors(&self) -> Vec<Pos> {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
            .iter()
            .map(|dir| dir.offset())
            .map(|offset| self.pos + offset)
            .collect()
    }

    fn flows(&self, other: &Tile) -> bool {
        let offset = other.pos - self.pos;
        let direction: Direction = offset.try_into().expect("valid offset");

        self.tile_type.flows(&direction, &other.tile_type)
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn offset(&self) -> Pos {
        match self {
            Direction::North => Pos::new(0, -1),
            Direction::South => Pos::new(0, 1),
            Direction::East => Pos::new(1, 0),
            Direction::West => Pos::new(-1, 0),
        }
    }
}

impl TryFrom<Pos> for Direction {
    type Error = Report;

    fn try_from(value: Pos) -> std::result::Result<Self, Self::Error> {
        if !(value.x.abs() == 1 && value.y == 0 || value.y.abs() == 1 && value.x == 0) {
            return Err(eyre!("offset is more than one step away or diagonal"));
        }

        Ok(match value {
            Pos { x: 0, y: -1 } => Direction::North,
            Pos { x: 0, y: 1 } => Direction::South,
            Pos { x: 1, y: 0 } => Direction::East,
            Pos { x: -1, y: 0 } => Direction::West,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TileType {
    Vertical,
    Horizontal,
    NorthEast90,
    NorthWest90,
    SouthWest90,
    SouthEast90,
    Ground,
    Start,
}

impl TileType {
    fn flows(&self, direction: &Direction, other: &TileType) -> bool {
        match self {
            TileType::Vertical => match direction {
                Direction::North => matches!(
                    other,
                    TileType::Vertical | TileType::SouthEast90 | TileType::SouthWest90
                ),
                Direction::South => matches!(
                    other,
                    TileType::Vertical | TileType::NorthEast90 | TileType::NorthWest90
                ),
                Direction::East | Direction::West => false,
            },
            TileType::Horizontal => match direction {
                Direction::North | Direction::South => false,
                Direction::East => matches!(
                    other,
                    TileType::Horizontal | TileType::NorthWest90 | TileType::SouthWest90
                ),
                Direction::West => matches!(
                    other,
                    TileType::Horizontal | TileType::NorthEast90 | TileType::SouthEast90
                ),
            },
            TileType::NorthEast90 => match direction {
                Direction::North => matches!(
                    other,
                    TileType::Vertical | TileType::SouthWest90 | TileType::SouthEast90
                ),
                Direction::East => matches!(
                    other,
                    TileType::Horizontal | TileType::NorthWest90 | TileType::SouthWest90
                ),
                Direction::South | Direction::West => false,
            },
            TileType::NorthWest90 => match direction {
                Direction::North => matches!(
                    other,
                    TileType::Vertical | TileType::SouthWest90 | TileType::SouthEast90
                ),
                Direction::West => matches!(
                    other,
                    TileType::Horizontal | TileType::NorthEast90 | TileType::SouthEast90
                ),
                Direction::South | Direction::East => false,
            },
            TileType::SouthWest90 => match direction {
                Direction::South => matches!(
                    other,
                    TileType::Vertical | TileType::NorthWest90 | TileType::NorthEast90
                ),
                Direction::West => matches!(
                    other,
                    TileType::Horizontal | TileType::NorthEast90 | TileType::SouthEast90
                ),
                Direction::North | Direction::East => false,
            },
            TileType::SouthEast90 => match direction {
                Direction::South => matches!(
                    other,
                    TileType::Vertical | TileType::NorthWest90 | TileType::NorthEast90
                ),
                Direction::East => matches!(
                    other,
                    TileType::Horizontal | TileType::NorthWest90 | TileType::SouthWest90
                ),
                Direction::North | Direction::West => false,
            },
            TileType::Start => match direction {
                Direction::North => matches!(
                    other,
                    TileType::Vertical | TileType::SouthWest90 | TileType::SouthEast90
                ),
                Direction::South => matches!(
                    other,
                    TileType::Vertical | TileType::NorthWest90 | TileType::NorthEast90
                ),
                Direction::East => matches!(
                    other,
                    TileType::Horizontal | TileType::NorthWest90 | TileType::SouthWest90
                ),
                Direction::West => matches!(
                    other,
                    TileType::Horizontal | TileType::NorthEast90 | TileType::SouthEast90
                ),
            },
            TileType::Ground => false,
        }
    }
}

impl Display for TileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            TileType::Vertical => '|',
            TileType::Horizontal => '-',
            TileType::NorthEast90 => 'L',
            TileType::NorthWest90 => 'J',
            TileType::SouthWest90 => '7',
            TileType::SouthEast90 => 'F',
            TileType::Ground => '.',
            TileType::Start => 'S',
        })
    }
}

mod parsers {
    use std::collections::HashMap;

    use nom::{
        character::complete::{line_ending, one_of},
        IResult,
        multi::{many1, separated_list1}, Parser,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{error::ErrorTree, final_parser::final_parser};

    use crate::day10::{Grid, Pos, Tile, TileType};

    type ParseError<'a> = ErrorTree<Span<'a>>;

    type Span<'a> = LocatedSpan<&'a str>;

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<Grid, ParseError> {
        let tiles = final_parser(tile_types)(Span::new(input))?;

        let map: HashMap<Pos, Tile> = tiles
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .map(|(col, tile)| {
                        let pos = Pos::new(col as i64, row as i64);

                        (
                            pos,
                            Tile {
                                tile_type: *tile,
                                pos,
                            },
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(Grid {
            tiles: map,
            rows: tiles.len(),
            cols: tiles.first().expect("there is a first row").len(),
        })
    }

    fn tile_types(input: Span) -> IResult<Span, Vec<Vec<TileType>>, ParseError> {
        separated_list1(line_ending, tile_type_line).parse(input)
    }

    fn tile_type_line(input: Span) -> IResult<Span, Vec<TileType>, ParseError> {
        many1(tile_type).parse(input)
    }

    fn tile_type(input: Span) -> IResult<Span, TileType, ParseError> {
        one_of("|-LJ7F.S")
            .map(|c| match c {
                '|' => TileType::Vertical,
                '-' => TileType::Horizontal,
                'L' => TileType::NorthEast90,
                'J' => TileType::NorthWest90,
                '7' => TileType::SouthWest90,
                'F' => TileType::SouthEast90,
                '.' => TileType::Ground,
                'S' => TileType::Start,
                _ => unreachable!(),
            })
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;
    use indoc::indoc;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::simple_map(indoc ! {
    ".....
     .S-7.
     .|.|.
     .L-J.
     ....."
    })]
    #[case::complex_map(indoc ! {
    "-L|F7
     7S-7|
     L|7||
     -L-J|
     L|-JF"
    })]
    fn test_parse_map(#[case] input: &str) -> Result<()> {
        let parsed_grid = input_generator(input)?;

        assert_eq!(input, parsed_grid.to_string().trim_end());

        Ok(())
    }

    #[rstest]
    #[case::simple_map(indoc ! {
    ".....
     .S-7.
     .|.|.
     .L-J.
     ....."
    }, indoc ! {
    ".....
     .S-7.
     .|.|.
     .L-J.
     ....."
    })]
    #[case::complex_map(indoc ! {
    "-L|F7
     7S-7|
     L|7||
     -L-J|
     L|-JF"
    }, indoc ! {
    ".....
     .S-7.
     .|.|.
     .L-J.
     ....."
    })]
    #[case::more_complex_main_loop(indoc ! {
    "7-F7-
     .FJ|7
     SJLL7
     |F--J
     LJ.LJ"
    }, indoc ! {
    "..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ..."
    })]
    fn test_find_main_loop(#[case] input: &str, #[case] expected_loop: &str) -> Result<()> {
        let parsed_grid = input_generator(input)?;
        let main_loop_grid = parsed_grid.main_loop()?;
        assert_eq!(expected_loop, main_loop_grid.to_string().trim_end());
        Ok(())
    }

    #[rstest]
    #[case::simple_map(indoc ! {
    ".....
     .S-7.
     .|.|.
     .L-J.
     ....."
    }, 4)]
    #[case::more_complex_main_loop(indoc ! {
    "7-F7-
     .FJ|7
     SJLL7
     |F--J
     LJ.LJ"
    }, 8)]
    fn test_part1(#[case] input: &str, #[case] expected_max: u64) -> Result<()> {
        let parsed_grid = input_generator(input)?;
        let max_distance = part1(&parsed_grid)?;

        assert_eq!(expected_max, max_distance);

        Ok(())
    }

    #[rstest]
    #[case::simple_map(indoc ! {
    ".....
     .S-7.
     .|.|.
     .L-J.
     ....."
    }, 1)]
    #[case::more_complex_main_loop(indoc ! {
    "...........
     .S-------7.
     .|F-----7|.
     .||.....||.
     .||.....||.
     .|L-7.F-J|.
     .|..|.|..|.
     .L--J.L--J.
     ..........."
    }, 4)]
    #[case::squeeze_through(indoc ! {
    "..........
     .S------7.
     .|F----7|.
     .||....||.
     .||....||.
     .|L-7F-J|.
     .|..||..|.
     .L--JL--J.
     .........."
    }, 4)]
    #[case::insane_case(indoc ! {
    "FF7FSF7F7F7F7F7F---7
     L|LJ||||||||||||F--J
     FL-7LJLJ||||||LJL-77
     F--JF--7||LJLJ7F7FJ-
     L---JF-JLJ.||-FJLJJ7
     |F|F-JF---7F7-L7L|7|
     |FFJF7L7F-JF7|JL---7
     7-L-JL7||F7|L7F-7F7|
     L.L7LFJ|||||FJL7||LJ
     L7JLJL-JLJLJL--JLJ.L"
    }, 10)]
    fn test_part2(#[case] input: &str, #[case] expected_area: u64) -> Result<()> {
        let parsed_grid = input_generator(input)?;
        let area = part2(&parsed_grid)?;

        assert_eq!(expected_area, area);

        Ok(())
    }
}
