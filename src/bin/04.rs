use std::collections::HashSet;

advent_of_code::solution!(4);

fn score_card(s: &str) -> u32 {
    let Some((_, card)) = s.split_once(':') else {
        panic!("missing : in card");
    };
    let Some((winning, have)) = card.split_once('|') else {
        panic!("missing | in card");
    };
    let check: HashSet<_> = winning.trim().split_ascii_whitespace().collect();

    let mut score = 0;
    for mine in have.trim().split_ascii_whitespace() {
        if check.contains(mine) {
            if score > 0 {
                score *= 2;
            } else {
                score = 1;
            }
        }
    }
    score
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|card| score_card(card)).sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_card() {
        let result = score_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
