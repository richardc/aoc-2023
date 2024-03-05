use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(3);

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    None,
    Digit(u8),
    Symbol(u8),
}

impl Cell {
    fn new(v: u8) -> Self {
        match v {
            b'0'..=b'9' => Cell::Digit(v - b'0'),
            b'.' => Cell::None,
            _ => Cell::Symbol(v),
        }
    }
}

struct Schematic {
    cells: Vec<Vec<Cell>>,
}

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Schematic {
            cells: s
                .lines()
                .map(|l| l.as_bytes().iter().map(|b| Cell::new(*b)).collect())
                .collect(),
        })
    }
}

#[derive(Debug)]
struct Serial {
    row: usize,
    column: usize,
    len: usize,
    value: u32,
}

impl Serial {
    fn is_used(&self, schematic: &Schematic) -> bool {
        for r in (self.row as i32 - 1)..=(self.row as i32 + 1) {
            for c in (self.column as i32 - 1)..=(self.column as i32 + self.len as i32) {
                if let Cell::Symbol(_) = schematic.get_cell(r, c) {
                    return true;
                }
            }
        }
        false
    }
}

impl Schematic {
    fn find_numbers(&self) -> Vec<Serial> {
        let mut result = Vec::new();
        for r in 0..self.cells.len() {
            let mut c: usize = 0;

            for (digit, cells) in &self.cells[r]
                .iter()
                .group_by(|a| matches!(a, Cell::Digit(_)))
            {
                let found = cells.collect_vec();
                if digit {
                    result.push(Serial {
                        row: r,
                        column: c,
                        len: found.len(),
                        value: found.iter().fold(0, |acc: u32, c| match c {
                            Cell::Digit(x) => acc * 10 + u32::from(*x),
                            _ => acc,
                        }),
                    });
                }
                c += found.len();
            }
        }
        result
    }

    fn get_cell(&self, r: i32, c: i32) -> Cell {
        if r < 0
            || c < 0
            || r as usize >= self.cells.len()
            || c as usize >= self.cells[r as usize].len()
        {
            return Cell::None;
        }
        self.cells[r as usize][c as usize]
    }

    fn part_numbers(&self) -> u32 {
        self.find_numbers()
            .iter()
            .filter_map(|serial| {
                if serial.is_used(self) {
                    Some(serial.value)
                } else {
                    None
                }
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic: Schematic = input.parse().unwrap();
    Some(schematic.part_numbers())
}

impl Serial {
    fn has_neighbour(&self, row: usize, col: usize) -> bool {
        for r in (self.row as i32 - 1)..=(self.row as i32 + 1) {
            for c in (self.column as i32 - 1)..=(self.column as i32 + self.len as i32) {
                if r == row as i32 && c == col as i32 {
                    return true;
                }
            }
        }
        false
    }
}

struct Gear {
    row: usize,
    column: usize,
}

impl Schematic {
    fn find_gears(&self) -> Vec<Gear> {
        let mut result = Vec::new();
        for r in 0..self.cells.len() {
            for c in 0..self.cells[r].len() {
                if let Cell::Symbol(b'*') = self.cells[r][c] {
                    result.push(Gear { row: r, column: c });
                }
            }
        }
        result
    }

    fn gear_ratios(&self) -> u32 {
        let mut result = 0;
        let serials = self.find_numbers();
        let gears = self.find_gears();
        for gear in gears {
            let touching = serials
                .iter()
                .filter(|s| s.has_neighbour(gear.row, gear.column))
                .collect_vec();
            if touching.len() == 2 {
                result += touching[0].value * touching[1].value;
            }
        }
        result
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic: Schematic = input.parse().unwrap();
    Some(schematic.gear_ratios())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_schematic_find_numbers() {
        let example = advent_of_code::template::read_file("examples", DAY);
        let schematic: Schematic = example.parse().unwrap();
        let result = schematic.find_numbers();
        assert_eq!(result.len(), 10);
        // println!("{:?}", result);
    }

    #[test]
    fn test_schematic_cell() {
        let example = advent_of_code::template::read_file("examples", DAY);
        let schematic: Schematic = example.parse().unwrap();

        assert_eq!(schematic.get_cell(0, 1), Cell::Digit(6));
        assert_eq!(schematic.get_cell(-1, -1), Cell::None);
        assert_eq!(schematic.get_cell(1, 3), Cell::Symbol(b'*'));
    }

    #[test]
    fn test_serial_is_used() {
        let example = advent_of_code::template::read_file("examples", DAY);
        let schematic: Schematic = example.parse().unwrap();

        let used = Serial {
            row: 0,
            column: 0,
            len: 3,
            value: 467,
        };
        assert_eq!(used.is_used(&schematic), true);

        let unused = Serial {
            row: 0,
            column: 5,
            len: 3,
            value: 114,
        };
        assert_eq!(unused.is_used(&schematic), false);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467_835));
    }
}
