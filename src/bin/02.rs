use std::{cmp::max, str::FromStr};

use anyhow::bail;

advent_of_code::solution!(2);

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game {
            id: 0,
            red: 0,
            green: 0,
            blue: 0,
        };

        if let Some((name, parts)) = s.split_once(':') {
            if let Some((_, id)) = name.split_once(' ') {
                game.id = id.parse().unwrap();

                for round in parts.trim().split(';') {
                    for die in round.trim().split(',') {
                        if let Some((count, color)) = die.trim().split_once(' ') {
                            let value = count.parse().unwrap();
                            match color {
                                "red" => game.red = max(game.red, value),
                                "green" => game.green = max(game.green, value),
                                "blue" => game.blue = max(game.blue, value),
                                _ => {}
                            }
                        } else {
                            bail!("die")
                        }
                    }
                }
                return Ok(game);
            }
        }
        bail!("game parse")
    }
}

impl Game {
    fn is_legal(&self, red: u32, green: u32, blue: u32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
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
        self.red * self.green * self.blue
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
