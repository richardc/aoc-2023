use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Default, Debug)]
struct Map {
    path: Vec<u8>,
    map: HashMap<[u8; 3], ([u8; 3], [u8; 3])>,
}

impl Map {
    fn new(s: &str) -> Result<Self, anyhow::Error> {
        let mut map = Self::default();
        for (i, line) in s.lines().enumerate() {
            if i == 0 {
                map.path = line.bytes().collect();
            }
            if i > 1 {
                map.map.insert(
                    line.as_bytes()[0..3].try_into()?,
                    (
                        line.as_bytes()[7..10].try_into()?,
                        line.as_bytes()[12..15].try_into()?,
                    ),
                );
            }
        }
        Ok(map)
    }

    fn steps(&self) -> usize {
        let mut current = b"AAA";
        let mut steps = 0;
        loop {
            let direction = self.path[steps % self.path.len()];
            let Some((l, r)) = self.map.get(current) else {
                unreachable!("bad step {:?}", current);
            };
            if direction == b'L' {
                current = l;
            } else {
                current = r;
            }
            steps += 1;

            if current == b"ZZZ" {
                return steps;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::new(input).unwrap();
    Some(map.steps())
}

impl Map {
    fn loops(&self, start: &[u8]) -> usize {
        let mut seen: HashMap<_, usize> = HashMap::new();
        let mut current = start;
        let mut steps = 0;
        loop {
            let direction = self.path[steps % self.path.len()];
            let Some((l, r)) = self.map.get(current) else {
                unreachable!();
            };
            if direction == b'L' {
                current = l;
            } else {
                current = r;
            }
            steps += 1;

            if current[2] == b'Z' {
                if let Some(last_here) = seen.get(current) {
                    return *last_here;
                }
                seen.insert(current, steps);
            }
        }
    }

    fn parallel_steps(&self) -> usize {
        let ghosts: Vec<_> = self.map.keys().filter(|k| k[2] == b'A').collect();
        ghosts
            .iter()
            .map(|&g| self.loops(g))
            .reduce(num_integer::lcm)
            .unwrap()
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::new(input).unwrap();
    Some(map.parallel_steps())
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
