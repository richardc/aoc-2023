use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(8);

#[derive(Default, Debug)]
struct Map {
    path: Vec<u8>,
    map: HashMap<String, (String, String)>,
}

impl Map {
    fn new(s: &str) -> Self {
        let mut map = Self::default();
        for (i, line) in s.lines().enumerate() {
            if i == 0 {
                map.path = line.bytes().collect();
            }
            if i > 1 {
                static RE: Lazy<Regex> = Lazy::new(|| {
                    Regex::new(r"^(?<node>.*?) = \((?<left>.*?), (?<right>.*?)\)").unwrap()
                });
                let Some(caps) = RE.captures(line) else {
                    panic!("regex")
                };

                map.map.insert(
                    caps["node"].to_string(),
                    (caps["left"].to_string(), caps["right"].to_string()),
                );
            }
        }
        map
    }

    fn steps(&self) -> usize {
        let mut current = &String::from("AAA");
        let dest = &String::from("ZZZ");
        let mut steps = 0;
        loop {
            let direction = self.path[steps % self.path.len()];
            let Some((l, r)) = self.map.get(current) else {
                unreachable!("bad step {}", current);
            };
            if direction == b'L' {
                current = l;
            } else {
                current = r;
            }
            steps += 1;

            if current == dest {
                return steps;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::new(input);
    Some(map.steps())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example1() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_example2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
