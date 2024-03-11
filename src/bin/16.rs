use itertools::Itertools;

use std::collections::HashSet;

advent_of_code::solution!(16);

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Vertical,
    Horizontal,
    Forward,
    Back,
}

impl Cell {
    fn new(b: u8) -> Self {
        use Cell::*;
        match b {
            b'.' => Empty,
            b'|' => Vertical,
            b'-' => Horizontal,
            b'/' => Forward,
            b'\\' => Back,
            _ => unreachable!("cell {}", b as char),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&mut self) {
        use Direction::*;
        *self = match *self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    fn turn_right(&mut self) {
        use Direction::*;
        *self = match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

struct Maze {
    data: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Maze {
    fn new(s: &str) -> Self {
        let data: Vec<Vec<Cell>> = s
            .lines()
            .map(|l| l.bytes().map(Cell::new).collect())
            .collect();
        let height = data.len();
        let width = data[0].len();
        Self {
            data,
            height,
            width,
        }
    }

    fn energised(&self) -> usize {
        use Cell::*;
        use Direction::*;
        let mut queue = vec![(0, 0, East)];
        let mut visited = HashSet::new();

        'path: while let Some((mut row, mut col, mut direction)) = queue.pop() {
            loop {
                if !visited.insert((row, col, direction)) {
                    continue 'path;
                };
                let cell = self.data[row][col];
                match (cell, &direction) {
                    (Empty, _)
                    | (Horizontal, East)
                    | (Horizontal, West)
                    | (Vertical, North)
                    | (Vertical, South) => {
                        // just keep moving
                    }
                    (Back, North) | (Back, South) | (Forward, West) | (Forward, East) => {
                        direction.turn_left();
                    }
                    (Back, West) | (Back, East) | (Forward, North) | (Forward, South) => {
                        direction.turn_right();
                    }
                    (Vertical, East)
                    | (Vertical, West)
                    | (Horizontal, North)
                    | (Horizontal, South) => {
                        let mut other_way = direction;
                        other_way.turn_left();
                        direction.turn_right();
                        queue.push((row, col, other_way));
                    }
                }

                match &direction {
                    North => {
                        if row == 0 {
                            continue 'path;
                        }
                        row -= 1;
                    }
                    South => {
                        row += 1;
                        if row == self.height {
                            continue 'path;
                        }
                    }
                    East => {
                        col += 1;
                        if col == self.width {
                            continue 'path;
                        }
                    }
                    West => {
                        if col == 0 {
                            continue 'path;
                        }
                        col -= 1;
                    }
                }
            }
        }

        visited.iter().unique_by(|(r, c, _)| (r, c)).count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(Maze::new(input).energised())
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
