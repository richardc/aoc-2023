use std::collections::HashSet;

advent_of_code::solution!(21);

struct Garden {
    walls: HashSet<(usize, usize)>,
    start: (usize, usize),
}

impl Garden {
    fn new(s: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start = (0, 0);
        for (r, row) in s.lines().enumerate() {
            for (c, b) in row.bytes().enumerate() {
                if b == b'#' {
                    walls.insert((r, c));
                }
                if b == b'S' {
                    start = (r, c)
                }
            }
        }
        Self { walls, start }
    }

    fn steps(&self, count: usize) -> usize {
        let mut steps_next: HashSet<(usize, usize)> = HashSet::new();
        let mut steps: HashSet<(usize, usize)> = HashSet::new();
        steps.insert(self.start);

        for _ in 0..count {
            steps_next.clear();
            for (r, c) in steps.drain() {
                for next in [(r + 1, c), (r, c + 1), (r - 1, c), (r, c - 1)] {
                    if self.walls.contains(&next) {
                        continue;
                    }
                    steps_next.insert(next);
                }
            }
            (steps, steps_next) = (steps_next, steps);
        }
        steps.len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(Garden::new(input).steps(64))
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let garden = Garden::new(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(garden.steps(6), 16);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
