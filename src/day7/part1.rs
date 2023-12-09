use aoc_runner_derive::{aoc, aoc_generator};
use color_eyre::{eyre::eyre, Result};
use itertools::Itertools;
use nom::{combinator::all_consuming, Finish};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(eyre!("unrecognized card type")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts = self.cards.iter().counts();
        let (&&max_card, &max_kind) = counts.iter().max_by(|(_, v), (_, v2)| v.cmp(v2)).unwrap();
        counts.remove(&max_card);

        match max_kind {
            5 => HandType::FiveKind,
            4 => HandType::FourKind,
            3 => {
                if counts.values().contains(&2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeKind
                }
            }
            2 => {
                if counts.values().contains(&2) {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        }
    }
}

impl TryFrom<&str> for Hand {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (_, hand) = all_consuming(parsers::hand)(value)
            .map_err(|e| e.to_owned())
            .finish()?;
        Ok(hand)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap().is_eq()
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_cmp = self.hand_type().cmp(&other.hand_type());

        if hand_cmp.is_eq() {
            self.cards
                .iter()
                .zip(&other.cards)
                .map(|(x, y)| x.cmp(y))
                .find_or_last(|cmp| !cmp.is_eq())
                .unwrap()
        } else {
            hand_cmp
        }
    }
}

#[aoc_generator(day7, part1)]
pub fn input_generator(input: &str) -> Result<Vec<Hand>> {
    parsers::parse_input(input)
}

#[aoc(day7, part1)]
pub fn part1(hands: &[Hand]) -> Result<usize> {
    Ok(hands
        .iter()
        .sorted()
        .enumerate()
        .rev()
        .map(|(idx, hand)| hand.bid as usize * (idx + 1))
        .sum())
}

mod parsers {
    use nom::{
        character,
        character::complete::{anychar, newline, space1},
        combinator::{all_consuming, map, map_res},
        multi::{many1, separated_list1},
        sequence::separated_pair,
        Finish, IResult,
    };

    use crate::day7::part1::{Card, Hand};

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<Vec<Hand>> {
        let (_, hands) = all_consuming(hands)(input)
            .map_err(|e| e.to_owned())
            .finish()?;
        Ok(hands)
    }

    fn hands(input: &str) -> IResult<&str, Vec<Hand>> {
        separated_list1(newline, hand)(input)
    }

    pub(crate) fn hand(input: &str) -> IResult<&str, Hand> {
        map(
            separated_pair(cards, space1, character::complete::u64),
            |(cards, bid)| Hand { cards, bid },
        )(input)
    }

    fn cards(input: &str) -> IResult<&str, Vec<Card>> {
        many1(card)(input)
    }

    fn card(input: &str) -> IResult<&str, Card> {
        map_res(anychar, Card::try_from)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day7::part1::parsers::parse_input;
    use color_eyre::Result;
    use rstest::rstest;

    const SAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_parser() -> Result<()> {
        let parser_output = input_generator(SAMPLE_INPUT)?;
        insta::assert_debug_snapshot!(parser_output);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = parse_input(SAMPLE_INPUT)?;
        let actual = part1(&input)?;

        assert_eq!(6440, actual);

        Ok(())
    }

    #[rstest]
    #[case("32T3K 765", HandType::OnePair)]
    #[case("T55J5 684", HandType::ThreeKind)]
    #[case("KK677 28", HandType::TwoPair)]
    #[case("KTJJT 220", HandType::TwoPair)]
    #[case("QQQJA 483", HandType::ThreeKind)]
    fn test_hand_type(#[case] hand: &str, #[case] hand_type: HandType) -> Result<()> {
        let hand: Hand = hand.try_into()?;
        assert_eq!(hand_type, hand.hand_type());
        Ok(())
    }
}
