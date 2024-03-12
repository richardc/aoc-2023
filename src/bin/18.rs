use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn new(b: u8) -> Self {
        use Direction::*;
        match b {
            b'U' | b'3' => Up,
            b'D' | b'1' => Down,
            b'R' | b'0' => Right,
            b'L' | b'2' => Left,
            _ => unreachable!("bad direction {}", b as char),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    distance: usize,
}

impl Instruction {
    fn new(s: &str) -> Self {
        let chunks = s.split(' ').collect_vec();
        let direction = Direction::new(chunks[0].as_bytes()[0]);
        let distance = chunks[1].parse().unwrap();
        Self {
            direction,
            distance,
        }
    }

    fn hex(s: &str) -> Self {
        let Some((_, hex)) = s.split_once('#') else {
            unreachable!("no hex code");
        };
        let direction = Direction::new(hex.as_bytes()[5]);
        let distance = usize::from_str_radix(&hex[0..5], 16).expect("bad hex");
        Self {
            direction,
            distance,
        }
    }
}

struct Digger {
    instructions: Vec<Instruction>,
}

impl Digger {
    fn new(s: &str) -> Self {
        let instructions = s.lines().map(Instruction::new).collect();
        Self { instructions }
    }

    fn new_hex(s: &str) -> Self {
        let instructions = s.lines().map(Instruction::hex).collect();
        Self { instructions }
    }

    fn cubic_meters(&self) -> usize {
        let mut points: Vec<(i64, i64)> = vec![(0, 0)];
        let mut r = 0;
        let mut c = 0;
        let mut edges = 0;
        use Direction::*;
        for step in &self.instructions {
            match step.direction {
                Right => c += step.distance,
                Left => c -= step.distance,
                Up => r -= step.distance,
                Down => r += step.distance,
            }
            edges += step.distance;
            points.push((r as i64, c as i64));
        }

        // https://en.wikipedia.org/wiki/Shoelace_formula
        // via https://stackoverflow.com/questions/24467972/calculate-area-of-polygon-given-x-y-coordinates
        let area = points
            .iter()
            .tuple_windows()
            .map(|((r1, c1), (r2, c2))| c1 * r2 - c2 * r1)
            .sum::<i64>()
            .abs()
            / 2;

        let result = area + edges as i64 / 2 + 1;
        result as usize
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let digger = Digger::new(input);
    Some(digger.cubic_meters())
}

pub fn part_two(input: &str) -> Option<usize> {
    let digger = Digger::new_hex(input);
    Some(digger.cubic_meters())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_hex_decoding() {
        use Direction::*;
        let digger = Digger::new_hex(&advent_of_code::template::read_file("examples", DAY));
        check!(
            digger.instructions[..2]
                == vec![
                    Instruction {
                        direction: Right,
                        distance: 461937
                    },
                    Instruction {
                        direction: Down,
                        distance: 56407
                    }
                ]
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952_408_144_115));
    }
}
