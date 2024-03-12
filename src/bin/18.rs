use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(Clone, Copy, Debug)]
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
            b'U' => Up,
            b'D' => Down,
            b'R' => Right,
            b'L' => Left,
            _ => unreachable!("bad direction {}", b as char),
        }
    }

    fn reverse(&self) -> Self {
        use Direction::*;
        match &self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

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
}

struct Digger {
    instructions: Vec<Instruction>,
}

impl Digger {
    fn new(s: &str) -> Self {
        let instructions = s.lines().map(Instruction::new).collect();
        Self { instructions }
    }

    fn cubic_meters(&self) -> usize {
        let mut dug_out: HashSet<(i32, i32)> = HashSet::new();

        let (mut r, mut c) = (0, 0);

        use Direction::*;

        dug_out.insert((0, 0));
        let mut path = Vec::new();
        for step in &self.instructions {
            for _ in 0..step.distance {
                match step.direction {
                    Right => c += 1,
                    Left => c -= 1,
                    Up => r -= 1,
                    Down => r += 1,
                }
                dug_out.insert((r, c));
                path.push((r, c, step.direction));
            }
        }

        let clockwise = &self
            .instructions
            .iter()
            .map(|i| i.direction)
            .tuple_windows()
            .map(|(from, to)| match (from, to) {
                (Up, Right) | (Right, Down) | (Down, Left) | (Left, Up) => 1,
                (Up, Left) | (Left, Down) | (Down, Right) | (Right, Up) => -1,
                _ => 0,
            })
            .sum::<i32>()
            > &0;

        let path = if clockwise {
            path
        } else {
            path.iter()
                .rev()
                .map(|&(r, c, d)| (r, c, d.reverse()))
                .collect()
        };

        fn flood_fill(row: i32, col: i32, hole: &mut HashSet<(i32, i32)>) {
            if hole.insert((row, col)) {
                flood_fill(row + 1, col, hole);
                flood_fill(row - 1, col, hole);
                flood_fill(row, col + 1, hole);
                flood_fill(row, col - 1, hole);
            }
        }
        for (r, c, d) in path {
            match d {
                Up => flood_fill(r, c + 1, &mut dug_out),
                Right => flood_fill(r + 1, c, &mut dug_out),
                Down => flood_fill(r, c - 1, &mut dug_out),
                Left => flood_fill(r - 1, c, &mut dug_out),
            }
        }

        dug_out.len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let digger = Digger::new(input);
    Some(digger.cubic_meters())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
