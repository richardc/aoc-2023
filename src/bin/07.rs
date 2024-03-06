use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use anyhow::bail;
use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Kind {
    FiveOfAKind = 10,
    FourOfAKind = 9,
    FullHouse = 8,
    ThreeOfAKind = 7,
    TwoPair = 6,
    OnePair = 5,
    HighCard = 4,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Value {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        match value {
            b'A' => Self::Ace,
            b'K' => Self::King,
            b'Q' => Self::Queen,
            b'J' => Self::Jack,
            b'T' => Self::Ten,
            b'9' => Self::Nine,
            b'8' => Self::Eight,
            b'7' => Self::Seven,
            b'6' => Self::Six,
            b'5' => Self::Five,
            b'4' => Self::Four,
            b'3' => Self::Three,
            b'2' => Self::Two,
            _ => unreachable!("{:?} not a card value", value),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Value>,
    kind: Kind,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            foo => foo,
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut histo = HashMap::new();
        for b in s.as_bytes() {
            histo.entry(b).and_modify(|x| *x += 1).or_insert(1);
        }
        let counts: Vec<_> = histo.values().sorted().rev().collect();
        Ok(Hand {
            cards: s.bytes().map(Value::from).collect(),
            kind: match counts.as_slice() {
                [5] => Kind::FiveOfAKind,
                [4, 1] => Kind::FourOfAKind,
                [3, 2] => Kind::FullHouse,
                [3, 1, 1] => Kind::ThreeOfAKind,
                [2, 2, 1] => Kind::TwoPair,
                [2, 1, 1, 1] => Kind::OnePair,
                _ => Kind::HighCard,
            },
        })
    }
}

#[derive(Debug, Default)]
struct Camel {
    hands: Vec<(Hand, u32)>,
}

impl FromStr for Camel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Self::default();
        for l in s.lines() {
            let Some((cards, bet)) = l.split_once(' ') else {
                bail!("input {}", l)
            };
            let hand: Hand = cards.parse()?;
            game.hands.push((hand, bet.parse()?));
        }
        Ok(game)
    }
}

impl Camel {
    fn total_winnings(&self) -> u32 {
        let mut winnings = 0;
        for (rank, (_hand, bid)) in self
            .hands
            .iter()
            .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .enumerate()
        {
            winnings += (rank + 1) as u32 * bid;
        }
        winnings
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let game: Camel = input.parse().unwrap();
    Some(game.total_winnings())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;
    use test_case::test_case;

    #[test]
    fn test_hand_fromstr() {
        let hand: Hand = "AAAAA".parse().unwrap();
        assert_eq!(
            hand,
            Hand {
                cards: vec![Value::Ace, Value::Ace, Value::Ace, Value::Ace, Value::Ace],
                kind: Kind::FiveOfAKind
            }
        );
    }

    #[test_case("AAAAA", Kind::FiveOfAKind)]
    #[test_case("AA2AA", Kind::FourOfAKind)]
    #[test_case("AA23A", Kind::ThreeOfAKind)]
    #[test_case("AAA22", Kind::FullHouse)]
    #[test_case("AA223", Kind::TwoPair)]
    #[test_case("AA324", Kind::OnePair)]
    #[test_case("23456", Kind::HighCard)]
    fn test_hand_kind(hand: &str, kind: Kind) {
        let result: Hand = hand.parse().unwrap();
        assert_eq!(result.kind, kind);
    }

    #[test_case("AAAAA", "22222")]
    #[test_case("AAAAA", "23456")]
    fn test_hand_beats(hand1: &str, hand2: &str) {
        let hand1: Hand = hand1.parse().unwrap();
        let hand2: Hand = hand2.parse().unwrap();
        check!(hand1 > hand2);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
