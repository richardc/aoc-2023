use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Empty, // .
    NS,    // |
    WE,    // -
    NE,    // L
    NW,    // J
    SW,    // 7,
    SE,    // F
    Start, // S
}

impl Tile {
    fn new(c: u8) -> Self {
        match c {
            b'.' => Self::Empty,
            b'|' => Self::NS,
            b'-' => Self::WE,
            b'L' => Self::NE,
            b'J' => Self::NW,
            b'7' => Self::SW,
            b'F' => Self::SE,
            b'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug)]
struct Maze {
    start: (i32, i32),
    cells: Vec<Vec<Tile>>,
}

impl Maze {
    fn new(s: &str) -> Self {
        let mut maze = Self::default();
        maze.cells = s
            .lines()
            .map(|l| l.bytes().map(Tile::new).collect())
            .collect();

        'scan: for (r, row) in maze.cells.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if *tile == Tile::Start {
                    maze.start = (r as i32, c as i32);
                    break 'scan;
                }
            }
        }

        maze.cells[maze.start.0 as usize][maze.start.1 as usize] = maze.start_tile();
        maze
    }

    fn get(&self, r: i32, c: i32) -> Tile {
        if r < 0
            || r as usize > self.cells.len()
            || c < 0
            || c as usize > self.cells[r as usize].len()
        {
            return Tile::Empty;
        }
        self.cells[r as usize][c as usize]
    }

    fn start_tile(&self) -> Tile {
        let north = self.get(self.start.0 - 1, self.start.1);
        let south = self.get(self.start.0 + 1, self.start.1);
        let west = self.get(self.start.0, self.start.1 - 1);
        let east = self.get(self.start.0, self.start.1 + 1);

        match (north, south, west, east) {
            // NS tiles north and south connect S N
            (Tile::NS, Tile::NS, _, _) => Tile::NS,

            (Tile::SE, Tile::NE, _, _) => Tile::NS,
            (Tile::SE, Tile::NW, _, _) => Tile::NS,

            (Tile::SW, Tile::NE, _, _) => Tile::NS,
            (Tile::SW, Tile::NW, _, _) => Tile::NS,

            // WE tiles west and east connect E W
            (_, _, Tile::WE, Tile::WE) => Tile::WE,

            (_, _, Tile::NE, Tile::WE) => Tile::WE,
            (_, _, Tile::SE, Tile::WE) => Tile::WE,

            (_, _, Tile::WE, Tile::NW) => Tile::WE,
            (_, _, Tile::WE, Tile::SW) => Tile::WE,

            // NW tiles north and west connect S E
            (Tile::NS, _, Tile::WE, _) => Tile::NW,

            (Tile::SW, _, Tile::WE, _) => Tile::NW,
            (Tile::SE, _, Tile::WE, _) => Tile::NW,

            (Tile::SW, _, Tile::NE, _) => Tile::NW,
            (Tile::SE, _, Tile::SE, _) => Tile::NW,

            // NE tiles north and east connect S W
            (Tile::NS, _, _, Tile::WE) => Tile::NE,

            (Tile::SW, _, _, Tile::WE) => Tile::NE,
            (Tile::SE, _, _, Tile::WE) => Tile::NE,

            (Tile::SW, _, _, Tile::NW) => Tile::NE,
            (Tile::SE, _, _, Tile::SW) => Tile::NE,

            // SW tiles south and east connect N E
            (_, Tile::NS, Tile::WE, _) => Tile::SW,

            (_, Tile::NW, Tile::WE, _) => Tile::SW,
            (_, Tile::NE, Tile::WE, _) => Tile::SW,

            (_, Tile::NS, Tile::NE, _) => Tile::SW,
            (_, Tile::NS, Tile::SE, _) => Tile::SW,

            // SE tiles south and west connect N W
            (_, Tile::NS, _, Tile::WE) => Tile::SE,

            (_, Tile::NW, _, Tile::WE) => Tile::SE,
            (_, Tile::NE, _, Tile::WE) => Tile::SE,

            (_, Tile::NS, _, Tile::NW) => Tile::SE,
            (_, Tile::NS, _, Tile::SW) => Tile::SE,

            _ => unreachable!(
                "can't deduce start kind ({:?}, {:?}, {:?}, {:?})",
                north, south, west, east
            ),
        }
    }

    fn connected(&self, r: i32, c: i32) -> Vec<(i32, i32)> {
        let tile = &self.cells[r as usize][c as usize];

        match tile {
            Tile::NS => vec![(r - 1, c), (r + 1, c)],
            Tile::WE => vec![(r, c - 1), (r, c + 1)],
            Tile::NE => vec![(r - 1, c), (r, c + 1)],
            Tile::NW => vec![(r - 1, c), (r, c - 1)],
            Tile::SE => vec![(r + 1, c), (r, c + 1)],
            Tile::SW => vec![(r + 1, c), (r, c - 1)],
            Tile::Empty => vec![],
            Tile::Start => unreachable!("start left in maze"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (i32, i32),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Maze {
    fn furthest_loop_distance(&self) -> usize {
        let mut dist: HashMap<(i32, i32), usize> = HashMap::new();
        let mut queue = BinaryHeap::new();
        dist.insert(self.start, 0);
        queue.push(State {
            cost: 0,
            position: self.start,
        });

        while let Some(State { cost, position }) = queue.pop() {
            if let Some(known) = dist.get(&position) {
                if cost > *known {
                    continue;
                }
            }

            for frontier in self.connected(position.0, position.1) {
                let next = State {
                    cost: cost + 1,
                    position: frontier,
                };
                if let Some(known) = dist.get(&frontier) {
                    if next.cost > *known {
                        continue;
                    }
                }
                queue.push(next);
                dist.insert(frontier, next.cost);
            }
        }

        *dist.values().max().unwrap()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = Maze::new(input);
    Some(maze.furthest_loop_distance())
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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_maze_start() {
        let maze = Maze::new(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(maze.start, (1, 1));
        assert_eq!(maze.start_tile(), Tile::SE);
    }

    #[test]
    fn test_part_one_complex() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
