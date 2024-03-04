use std::str::FromStr;

use anyhow::bail;

advent_of_code::solution!(2);

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r = Round {
            red: 0,
            green: 0,
            blue: 0,
        };

        for die in s.trim().split(',') {
            if let Some((count, color)) = die.trim().split_once(' ') {
                match color {
                    "red" => r.red = count.parse().unwrap(),
                    "green" => r.green = count.parse().unwrap(),
                    "blue" => r.blue = count.parse().unwrap(),
                    _ => {}
                }
            } else {
                bail!("parsing die {}", die)
            }
        }
        Ok(r)
    }
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((game, parts)) = s.split_once(':') {
            if let Some((_, id)) = game.split_once(' ') {
                Ok(Game {
                    id: id.parse().unwrap(),
                    rounds: parts.split(';').map(|r| r.parse().unwrap()).collect(),
                })
            } else {
                bail!("Bad format")
            }
        } else {
            bail!("Games")
        }
    }
}

impl Game {
    fn is_legal(&self, red: u32, green: u32, blue: u32) -> bool {
        self.rounds
            .iter()
            .all(|r| r.red <= red && r.green <= green && r.blue <= blue)
    }
}

fn parse(input: &str) -> Vec<Game> {
    input.lines().map(|l| Game::from_str(l).unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse(input);
    let legal = games.iter().filter_map(|x| {
        if x.is_legal(12, 13, 14) {
            Some(x.id)
        } else {
            None
        }
    });
    Some(legal.sum())
}

impl Game {
    fn power(&self) -> u32 {
        let r = self.rounds.iter().map(|r| r.red).max().unwrap();
        let g = self.rounds.iter().map(|r| r.green).max().unwrap();
        let b = self.rounds.iter().map(|r| r.blue).max().unwrap();
        r * g * b
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse(input);
    Some(games.iter().map(|g| g.power()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
