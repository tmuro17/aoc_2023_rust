use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Scratcher {
    pub id: u64,
    pub winning_numbers: Vec<u64>,
    pub play_numbers: Vec<u64>,
}

#[derive(Debug, Default, Clone)]
pub struct Scratchers(Vec<Scratcher>);

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> color_eyre::Result<Vec<Scratcher>> {
    parsers::parse_input(input)
}

impl Scratcher {
    fn points(&self) -> u64 {
        let num_winners = self.num_winning();

        if num_winners == 0 {
            0
        } else {
            2_u64.pow((num_winners - 1) as u32)
        }
    }

    fn num_winning(&self) -> u64 {
        self.play_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count() as u64
    }
}

impl Scratchers {
    fn get_rewards(&self, winner_id: u64, count: u64) -> Scratchers {
        let id = winner_id as usize;
        let offset = id + count as usize;
        self.0[id..offset].to_vec().into()
    }
}

impl From<Vec<Scratcher>> for Scratchers {
    fn from(scratchers: Vec<Scratcher>) -> Self {
        Scratchers(scratchers)
    }
}

#[aoc(day4, part1)]
pub fn part1(scratchers: &[Scratcher]) -> u64 {
    scratchers.iter().map(|scratcher| scratcher.points()).sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Scratcher]) -> u64 {
    let mut memo: HashMap<u64, u64> = HashMap::new();
    let scratchers: Scratchers = input.to_vec().into();
    let hand = scratchers.clone().0;

    hand.iter()
        .map(|card| evaluate(&scratchers, card, &mut memo))
        .sum()
}

fn evaluate(scratchers: &Scratchers, scratcher: &Scratcher, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(count) = memo.get(&scratcher.id) {
        return *count;
    }

    let winnings = scratchers.get_rewards(scratcher.id, scratcher.num_winning());

    let count = 1 + winnings
        .0
        .iter()
        .map(|card| evaluate(scratchers, card, memo))
        .sum::<u64>();

    memo.insert(scratcher.id, count);

    count
}

mod parsers {
    use nom::{
        bytes::complete::tag,
        character,
        character::complete::{newline, space0, space1},
        combinator::{all_consuming, map},
        multi::separated_list1,
        sequence::{delimited, preceded, separated_pair, terminated, tuple},
        Finish, IResult,
    };

    use crate::day4::Scratcher;

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<Vec<Scratcher>> {
        let (_, scratchers) = all_consuming(scratchers)(input)
            .map_err(|e| e.to_owned())
            .finish()?;
        Ok(scratchers)
    }

    fn scratchers(input: &str) -> IResult<&str, Vec<Scratcher>> {
        separated_list1(newline, scratcher)(input)
    }

    pub(crate) fn scratcher(input: &str) -> IResult<&str, Scratcher> {
        map(
            tuple((
                terminated(
                    preceded(terminated(tag("Card"), space0), character::complete::u64),
                    terminated(tag(":"), space0),
                ),
                separated_pair(
                    number_list,
                    delimited(space0, tag("|"), space0),
                    number_list,
                ),
            )),
            |(id, (winning, play))| Scratcher {
                id,
                winning_numbers: winning,
                play_numbers: play,
            },
        )(input)
    }

    fn number_list(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list1(space1, character::complete::u64)(input)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::day4::{parsers, parsers::parse_input, part1, part2, Scratcher};

    static INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn parse_sample_input() {
        let expected = vec![
            Scratcher {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                play_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Scratcher {
                id: 2,
                winning_numbers: vec![13, 32, 20, 16, 61],
                play_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
            Scratcher {
                id: 3,
                winning_numbers: vec![1, 21, 53, 59, 44],
                play_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            },
            Scratcher {
                id: 4,
                winning_numbers: vec![41, 92, 73, 84, 69],
                play_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
            },
            Scratcher {
                id: 5,
                winning_numbers: vec![87, 83, 26, 28, 32],
                play_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
            },
            Scratcher {
                id: 6,
                winning_numbers: vec![31, 18, 13, 56, 72],
                play_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
            },
        ];

        let actual = parse_input(INPUT).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_sample_input() {
        let input = parse_input(INPUT).unwrap();

        let actual = part1(&input);
        assert_eq!(13, actual);
    }
    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn points_tests(#[case] card: &str, #[case] point: u64) {
        let (_, scratcher) = parsers::scratcher(card).unwrap();

        assert_eq!(point, scratcher.points());
    }

    #[test]
    fn part2_sample_input() {
        let input = parse_input(INPUT).unwrap();
        let actual = part2(&input);
        assert_eq!(30, actual);
    }
}
