use std::{cmp::Ordering, collections::HashMap};

use anyhow::bail;
use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Value {
    Joker,
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
    rank: Rank,
    jokers_wild: bool,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.rank.partial_cmp(&other.rank) {
            Some(Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            rank => rank,
        }
    }
}

impl Hand {
    fn from_str(s: &str, jokers_wild: bool) -> Result<Self, anyhow::Error> {
        let mut histo = HashMap::new();
        for b in s.as_bytes() {
            histo.entry(b).and_modify(|x| *x += 1).or_insert(1);
        }
        if jokers_wild {
            if let Some((_, num_jokers)) = histo.remove_entry(&b'J') {
                if num_jokers == 5 {
                    // oops, put that back
                    histo.insert(&b'J', 5);
                } else {
                    let largest = histo.iter().max_by_key(|e| e.1).unwrap().0;
                    histo.entry(largest).and_modify(|x| *x += num_jokers);
                }
            };
        }
        let counts: Vec<_> = histo.values().sorted().rev().collect();
        Ok(Hand {
            cards: s
                .bytes()
                .map(Value::from)
                .map(|v| {
                    if jokers_wild && v == Value::Jack {
                        Value::Joker
                    } else {
                        v
                    }
                })
                .collect(),
            rank: match counts.as_slice() {
                [5] => Rank::FiveOfAKind,
                [4, 1] => Rank::FourOfAKind,
                [3, 2] => Rank::FullHouse,
                [3, 1, 1] => Rank::ThreeOfAKind,
                [2, 2, 1] => Rank::TwoPair,
                [2, 1, 1, 1] => Rank::OnePair,
                _ => Rank::HighCard,
            },
            jokers_wild: jokers_wild,
        })
    }
}

#[derive(Debug, Default)]
struct Camel {
    hands: Vec<(Hand, u32)>,
}

impl Camel {
    fn from_str(s: &str, jokers_wild: bool) -> Result<Self, anyhow::Error> {
        let mut game = Self::default();
        for l in s.lines() {
            let Some((cards, bet)) = l.split_once(' ') else {
                bail!("input {}", l)
            };
            let hand: Hand = Hand::from_str(cards, jokers_wild)?;
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
    let game = Camel::from_str(input, false).unwrap();
    Some(game.total_winnings())
}

pub fn part_two(input: &str) -> Option<u32> {
    let game = Camel::from_str(input, true).unwrap();
    Some(game.total_winnings())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;
    use test_case::test_case;

    #[test]
    fn test_hand_fromstr() {
        let hand: Hand = Hand::from_str("AAAAA", false).unwrap();
        assert_eq!(
            hand,
            Hand {
                cards: vec![Value::Ace, Value::Ace, Value::Ace, Value::Ace, Value::Ace],
                rank: Rank::FiveOfAKind,
                jokers_wild: false,
            }
        );
    }

    #[test_case("AAAAA", false, Rank::FiveOfAKind)]
    #[test_case("AAJAA", false, Rank::FourOfAKind)]
    #[test_case("AAJ3A", false, Rank::ThreeOfAKind)]
    #[test_case("AAAJJ", false, Rank::FullHouse)]
    #[test_case("AA22J", false, Rank::TwoPair)]
    #[test_case("AA32J", false, Rank::OnePair)]
    #[test_case("2345J", false, Rank::HighCard)]
    #[test_case("AAAAA", true, Rank::FiveOfAKind)]
    #[test_case("AAJAA", true, Rank::FiveOfAKind)]
    #[test_case("AAJ3A", true, Rank::FourOfAKind)]
    #[test_case("AAAJJ", true, Rank::FiveOfAKind)]
    #[test_case("AA22J", true, Rank::FullHouse)]
    #[test_case("AA32J", true, Rank::ThreeOfAKind)]
    #[test_case("2345J", true, Rank::OnePair)]
    fn test_hand_kind(hand: &str, jokers_wild: bool, kind: Rank) {
        let result = Hand::from_str(hand, jokers_wild).unwrap();
        assert_eq!(result.rank, kind);
    }

    #[test_case("AAAAA", "22222")]
    #[test_case("AAAAA", "23456")]
    fn test_hand_beats(hand1: &str, hand2: &str) {
        let hand1 = Hand::from_str(hand1, false).unwrap();
        let hand2 = Hand::from_str(hand2, false).unwrap();
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
        assert_eq!(result, Some(5905));
    }
}
