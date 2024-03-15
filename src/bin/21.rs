use std::collections::HashSet;

advent_of_code::solution!(21);

#[derive(Debug)]
struct Garden {
    walls: HashSet<(i32, i32)>,
    width: i32,
    height: i32,
    start: (i32, i32),
}

impl Garden {
    fn new(s: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start = (0, 0);
        for (r, row) in s.lines().enumerate() {
            for (c, b) in row.bytes().enumerate() {
                if b == b'#' {
                    walls.insert((r as i32, c as i32));
                }
                if b == b'S' {
                    start = (r as i32, c as i32)
                }
            }
        }
        let width = s.lines().next().unwrap().len() as i32;
        let height = s.lines().count() as i32;
        Self {
            walls,
            start,
            width,
            height,
        }
    }

    fn steps(&self, count: usize) -> usize {
        let mut steps_next: HashSet<(i32, i32)> = HashSet::new();
        let mut steps: HashSet<(i32, i32)> = HashSet::new();
        steps.insert(self.start);

        for _ in 0..count {
            steps_next.clear();
            for (r, c) in steps.drain() {
                for next in [(r + 1, c), (r, c + 1), (r - 1, c), (r, c - 1)] {
                    let wrapped = (
                        next.0.rem_euclid(self.height),
                        next.1.rem_euclid(self.width),
                    );
                    if self.walls.contains(&wrapped) {
                        continue;
                    }
                    steps_next.insert(next);
                }
            }
            (steps, steps_next) = (steps_next, steps);
        }
        steps.len()
    }

    fn many_steps(&self, counts: &[usize]) -> Vec<isize> {
        let mut results = Vec::new();
        let mut steps_next: HashSet<(i32, i32)> = HashSet::new();
        let mut steps: HashSet<(i32, i32)> = HashSet::new();
        steps.insert(self.start);

        let mut iteration = 0;
        'counting: for count in counts {
            loop {
                iteration += 1;
                steps_next.clear();
                for (r, c) in steps.drain() {
                    for next in [(r + 1, c), (r, c + 1), (r - 1, c), (r, c - 1)] {
                        let wrapped = (
                            next.0.rem_euclid(self.height),
                            next.1.rem_euclid(self.width),
                        );
                        if self.walls.contains(&wrapped) {
                            continue;
                        }
                        steps_next.insert(next);
                    }
                }
                (steps, steps_next) = (steps_next, steps);
                if *count == iteration {
                    results.push(steps.len() as isize);
                    continue 'counting;
                }
            }
        }

        results
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(Garden::new(input).steps(64))
}

impl Garden {
    fn steps_quadratic(&self, steps: usize) -> usize {
        let n = (steps / self.width as usize) as isize;
        let remainder = steps % self.width as usize;

        assert!(remainder == (self.width / 2) as usize);

        let values = self.many_steps(
            &(0..=2)
                .map(|i| remainder + i * self.width as usize)
                .collect::<Vec<_>>(),
        );
        let [v0, v1, v2] = values[..] else {
            unreachable!("have 3 thingies")
        };

        let a = (v0 - 2 * v1 + v2) / 2;
        let b = (-3 * v0 + 4 * v1 - v2) / 2;
        let c = v0;

        // a * n^2 + bn + c
        (a * n * n + b * n + c) as usize
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Garden::new(input).steps_quadratic(26_501_365))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_part_one() {
        let garden = Garden::new(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(garden.steps(6), 16);
    }

    #[test_case(6 => 16)]
    #[test_case(10 => 50)]
    #[test_case(50 => 1594)]
    fn test_part_two(steps: usize) -> usize {
        let garden = Garden::new(&advent_of_code::template::read_file("examples", DAY));
        garden.steps(steps)
    }
}
