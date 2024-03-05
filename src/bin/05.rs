use std::str::FromStr;

advent_of_code::solution!(5);

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
struct Mapping {
    destination: u64,
    source: u64,
    length: u64,
}

impl Mapping {
    fn apply(&self, from: u64) -> Option<u64> {
        if from >= self.source && from <= self.source + self.length {
            Some(from - self.source + self.destination)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Mapper {
    maps: Vec<Mapping>,
}

impl Mapper {
    fn map(&self, from: u64) -> u64 {
        for map in &self.maps {
            if let Some(v) = map.apply(from) {
                return v;
            }
        }
        from
    }
}

#[derive(Default, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    steps: Vec<Mapper>,
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut almanac = Self::default();

        for line in s.lines() {
            static SEED_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^seeds: (.*)").unwrap());
            if let Some(caps) = SEED_RE.captures(line) {
                almanac.seeds = caps[1]
                    .split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
            }

            static MAP_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.*)-to-(.*) map:").unwrap());
            if let Some(_caps) = MAP_RE.captures(line) {
                almanac.steps.push(Mapper { maps: Vec::new() })
            }

            static LINE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+) (\d+) (\d+)").unwrap());
            if let Some(caps) = LINE_RE.captures(line) {
                let i = almanac.steps.len() - 1;
                almanac.steps[i].maps.push(Mapping {
                    destination: caps[1].parse().unwrap(),
                    source: caps[2].parse()?,
                    length: caps[3].parse()?,
                })
            }
        }
        Ok(almanac)
    }
}

impl Almanac {
    fn map(&self, seed: u64) -> u64 {
        self.steps.iter().fold(seed, |acc, m| m.map(acc))
    }

    fn lowest_location(&self) -> u64 {
        self.seeds.iter().map(|seed| self.map(*seed)).min().unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac: Almanac = input.parse().unwrap();
    Some(almanac.lowest_location())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_apply() {
        let almanac: Almanac = advent_of_code::template::read_file("examples", DAY)
            .parse()
            .unwrap();
        assert_eq!(almanac.steps[0].map(98), 50);
        assert_eq!(almanac.steps[0].map(99), 51);
        assert_eq!(almanac.steps[0].map(53), 55);
        assert_eq!(almanac.steps[0].map(10), 10);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
