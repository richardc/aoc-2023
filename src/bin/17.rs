use pathfinding::prelude::dijkstra;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(17);

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn reverse(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct Cart {
    row: usize,
    column: usize,
    direction: Option<Direction>,
    run: usize,
}

impl Cart {
    fn new(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
            ..Default::default()
        }
    }

    fn position(&self) -> (usize, usize) {
        (self.row, self.column)
    }

    fn go(
        &self,
        direction: Direction,
        can_turn: fn(usize) -> bool,
        must_turn: fn(usize) -> bool,
        maze: &Maze,
    ) -> Option<Self> {
        use Direction::*;
        let mut next = self.clone();
        next.direction = Some(direction);

        // Track and reject based on where we've been
        if let Some(heading) = &self.direction {
            if *heading == direction.reverse() {
                // we can only go straight on, left, or right
                return None;
            }

            if *heading == direction {
                // continue forward
                next.run += 1;

                // unless we've being going forward too long
                if must_turn(next.run) {
                    return None;
                }
            } else {
                // a turn, are we allowed it?
                if can_turn(next.run) {
                    next.run = 0;
                } else {
                    return None;
                }
            }
        }

        // compute position or None if it'll take us out of bounds
        match direction {
            North => {
                if self.row == 0 {
                    return None;
                }
                next.row -= 1;
            }
            South => {
                if self.row == maze.height - 1 {
                    return None;
                }
                next.row += 1;
            }
            West => {
                if self.column == 0 {
                    return None;
                }
                next.column -= 1;
            }
            East => {
                if self.column == maze.width - 1 {
                    return None;
                }
                next.column += 1;
            }
        }

        Some(next)
    }

    fn successors(
        &self,
        can_turn: fn(usize) -> bool,
        must_turn: fn(usize) -> bool,
        maze: &Maze,
    ) -> Vec<(Self, u32)> {
        let neighbours = Direction::iter().filter_map(|d| self.go(d, can_turn, must_turn, maze));
        neighbours
            .map(|cart| {
                let cost = maze.cost(cart.position());
                (cart, cost)
            })
            .collect()
    }
}

struct Maze {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Maze {
    fn new(s: &str) -> Self {
        let data: Vec<Vec<_>> = s
            .lines()
            .map(|l| l.bytes().map(|b| b - b'0').collect())
            .collect();
        let height = data.len();
        let width = data[0].len();
        Self {
            data,
            height,
            width,
        }
    }

    fn cost(&self, position: (usize, usize)) -> u32 {
        self.data[position.0][position.1] as u32
    }

    fn min_heat_loss(&self, can_turn: fn(usize) -> bool, must_turn: fn(usize) -> bool) -> u32 {
        let goal = (self.height - 1, self.width - 1);
        let Some((_, cost)) = dijkstra(
            &Cart::new(0, 0),
            |cart| cart.successors(can_turn, must_turn, self),
            |cart| cart.position() == goal,
        ) else {
            unreachable!("should have a path");
        };

        cost
    }

    fn crucible_heat_loss(&self) -> u32 {
        let can_turn = |_| true;
        let must_turn = |step| step >= 3;

        self.min_heat_loss(can_turn, must_turn)
    }

    fn ultracrucible_heat_loss(&self) -> u32 {
        let can_turn = |step| step >= 3;
        let must_turn = |step| step >= 10;

        self.min_heat_loss(can_turn, must_turn)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = Maze::new(input);
    Some(maze.crucible_heat_loss())
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = Maze::new(input);
    Some(maze.ultracrucible_heat_loss())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
