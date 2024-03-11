use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Ball,
    Pillar,
    Empty,
}

impl Tile {
    fn new(b: u8) -> Self {
        match b {
            b'O' => Tile::Ball,
            b'#' => Tile::Pillar,
            b'.' => Tile::Empty,
            b => unreachable!("unknown tile {}", b as char),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Tile::Ball => 'O',
                Tile::Pillar => '#',
                Tile::Empty => '.',
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Rocks {
    data: Vec<Vec<Tile>>,
}

impl Display for Rocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for v in row {
                write!(f, "{}", v)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Rocks {
    fn new(s: &str) -> Self {
        Self {
            data: s
                .lines()
                .map(|l| l.bytes().map(Tile::new).collect())
                .collect(),
        }
    }

    fn roll_north(&mut self) {
        for col in 0..self.data[0].len() {
            let mut newcol: Vec<Tile> = Vec::new();
            for (pillar, group) in &(0..self.data.len())
                .map(|r| self.data[r][col])
                .group_by(|&t| t == Tile::Pillar)
            {
                if pillar {
                    newcol.extend(group)
                } else {
                    // sorting the group will make all the Ball 'roll' up before Empty
                    newcol.extend(group.sorted())
                }
            }
            for (r, &v) in newcol.iter().enumerate() {
                self.data[r][col] = v
            }
        }
    }

    fn roll_south(&mut self) {
        for col in 0..self.data[0].len() {
            let mut newcol: Vec<Tile> = Vec::new();
            for (pillar, group) in &(0..self.data.len())
                .map(|r| self.data[r][col])
                .group_by(|&t| t == Tile::Pillar)
            {
                if pillar {
                    newcol.extend(group)
                } else {
                    // sorting the group will make all the Ball 'roll' up before Empty
                    newcol.extend(group.sorted().rev())
                }
            }
            for (r, &v) in newcol.iter().enumerate() {
                self.data[r][col] = v
            }
        }
    }

    fn roll_west(&mut self) {
        for row in 0..self.data.len() {
            let mut newrow: Vec<Tile> = Vec::new();
            for (pillar, group) in &self.data[row].iter().group_by(|t| **t == Tile::Pillar) {
                if pillar {
                    newrow.extend(group)
                } else {
                    // sorting the group will make all the Ball 'roll' up before Empty
                    newrow.extend(group.sorted())
                }
            }
            self.data[row] = newrow;
        }
    }

    fn roll_east(&mut self) {
        for row in 0..self.data.len() {
            let mut newrow: Vec<Tile> = Vec::new();
            for (pillar, group) in &self.data[row].iter().group_by(|t| **t == Tile::Pillar) {
                if pillar {
                    newrow.extend(group)
                } else {
                    // sorting the group will make all the Ball 'roll' up before Empty
                    newrow.extend(group.sorted().rev())
                }
            }
            self.data[row] = newrow;
        }
    }

    fn spin(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn north_weight(&self) -> usize {
        let depth = self.data.len();
        let mut result = 0;
        for (r, row) in self.data.iter().enumerate() {
            result += row.iter().filter(|t| **t == Tile::Ball).count() * (depth - r);
        }
        result
    }

    fn spin_cycle(&mut self) -> usize {
        let target = 1_000_000_000;
        let mut current = 0;
        let mut cache: HashMap<Vec<Vec<Tile>>, usize> = HashMap::new();
        while current < target {
            self.spin();
            current += 1;
            if let Some(last) = cache.insert(self.data.clone(), current) {
                //println!("cycle found at {last} from {current}");
                let remaining = target - current;
                let step = current - last;
                let steps_left = remaining / step;
                current += step * steps_left;
                //println!("resuming from {current}");
                break;
            }
        }
        while current < target {
            self.spin();
            current += 1;
        }
        self.north_weight()
    }
}

pub fn spin_demo(input: &str) {
    let mut rocks = Rocks::new(input);

    println!("start:\n{}", rocks);
    rocks.roll_north();
    println!("north:\n{}", rocks);
    rocks.roll_east();
    println!("east:\n{}", rocks);
    rocks.roll_south();
    println!("south:\n{}", rocks);
    rocks.roll_west();
    println!("west:\n{}", rocks);
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut rocks = Rocks::new(input);
    rocks.roll_north();
    Some(rocks.north_weight())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut rocks = Rocks::new(input);
    Some(rocks.spin_cycle())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
