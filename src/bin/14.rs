use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug)]
struct Rocks {
    data: Vec<Vec<Tile>>,
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

    fn north_weight(&self) -> usize {
        let depth = self.data.len();
        let mut result = 0;
        for (r, row) in self.data.iter().enumerate() {
            result += row.iter().filter(|t| **t == Tile::Ball).count() * (depth - r);
        }
        result
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut rocks = Rocks::new(input);
    rocks.roll_north();
    Some(rocks.north_weight())
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
