use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::eyre::Result;
use std::cmp::max;
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Game>> {
    parsers::parse_input(input)
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Game {
    id: u64,
    pulls: Vec<Pull>,
}

#[derive(Debug, PartialOrd, PartialEq, Default)]
struct Pull {
    red: u64,
    green: u64,
    blue: u64,
}

#[aoc(day2, part1)]
pub fn part1(input: &[Game]) -> u64 {
    input
        .iter()
        .filter(|game| {
            !game
                .pulls
                .iter()
                .any(|pull| pull.red > 12 || pull.green > 13 || pull.blue > 14)
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Game]) -> u64 {
    input
        .iter()
        .map(|game| {
            game.pulls.iter().fold(Pull::default(), |acc, pull| Pull {
                red: max(acc.red, pull.red),
                green: max(acc.green, pull.green),
                blue: max(acc.blue, pull.blue),
            })
        })
        .map(|max_pull| max_pull.red * max_pull.green * max_pull.blue)
        .sum()
}

mod parsers {
    use crate::day2::{Game, Pull};
    use nom::{
        character::complete::newline, combinator::all_consuming, multi::separated_list1, Finish,
        IResult,
    };

    use color_eyre::eyre::Result;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{space0, space1, u64},
        combinator::map,
        sequence::{preceded, terminated, tuple},
    };

    enum Color {
        Red,
        Green,
        Blue,
    }

    struct ColorEntry {
        color: Color,
        quantity: u64,
    }

    pub(crate) fn parse_input(input: &str) -> Result<Vec<Game>> {
        let (_, games) = all_consuming(games)(input)
            .map_err(|e| e.to_owned())
            .finish()?;
        Ok(games)
    }

    fn games(input: &str) -> IResult<&str, Vec<Game>> {
        separated_list1(newline, game_line)(input)
    }

    fn game_line(input: &str) -> IResult<&str, Game> {
        let (input, id) = preceded(
            terminated(tag("Game"), space1),
            terminated(u64, terminated(tag(":"), space0)),
        )(input)?;
        let (input, pulls) = separated_list1(terminated(tag(";"), space0), pull)(input)?;

        Ok((input, Game { id, pulls }))
    }

    fn pull(input: &str) -> IResult<&str, Pull> {
        map(separated_list1(tag(", "), color_entry), |entries| {
            let mut pull = Pull::default();

            entries.iter().for_each(|entry| match entry.color {
                Color::Red => pull.red += entry.quantity,
                Color::Green => pull.green += entry.quantity,
                Color::Blue => pull.blue += entry.quantity,
            });

            pull
        })(input)
    }

    fn color_entry(input: &str) -> IResult<&str, ColorEntry> {
        map(tuple((u64, space1, color)), |(quantity, _, color)| {
            ColorEntry { color, quantity }
        })(input)
    }

    fn color(input: &str) -> IResult<&str, Color> {
        alt((
            map(tag("blue"), |_| Color::Blue),
            map(tag("red"), |_| Color::Red),
            map(tag("green"), |_| Color::Green),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let games = input_generator(input).unwrap();

        let expected_games = vec![
            Game {
                id: 1,
                pulls: vec![
                    Pull {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Pull {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Pull {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 2,
                pulls: vec![
                    Pull {
                        red: 0,
                        green: 2,
                        blue: 1,
                    },
                    Pull {
                        red: 1,
                        green: 3,
                        blue: 4,
                    },
                    Pull {
                        red: 0,
                        green: 1,
                        blue: 1,
                    },
                ],
            },
            Game {
                id: 3,
                pulls: vec![
                    Pull {
                        red: 20,
                        green: 8,
                        blue: 6,
                    },
                    Pull {
                        red: 4,
                        green: 13,
                        blue: 5,
                    },
                    Pull {
                        red: 1,
                        green: 5,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 4,
                pulls: vec![
                    Pull {
                        red: 3,
                        green: 1,
                        blue: 6,
                    },
                    Pull {
                        red: 6,
                        green: 3,
                        blue: 0,
                    },
                    Pull {
                        red: 14,
                        green: 3,
                        blue: 15,
                    },
                ],
            },
            Game {
                id: 5,
                pulls: vec![
                    Pull {
                        red: 6,
                        green: 3,
                        blue: 1,
                    },
                    Pull {
                        red: 1,
                        green: 2,
                        blue: 2,
                    },
                ],
            },
        ];

        assert_eq!(games, expected_games);

        let output = part1(&games);
        let expected_output = 8;

        assert_eq!(output, expected_output);
    }
    #[test]
    fn part2_example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let games = input_generator(input).unwrap();

        let output = part2(&games);
        let expected_output = 2286;

        assert_eq!(output, expected_output);
    }
}
