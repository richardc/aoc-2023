use pathfinding::directed::bfs::bfs_loop;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Maze {
    fn new(s: &str) -> Self {
        let mut maze = Self {
            cells: s
                .lines()
                .map(|l| l.bytes().map(Tile::new).collect())
                .collect(),
            ..Default::default()
        };

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

    fn next_step(&self, r: i32, c: i32, facing: Direction) -> (i32, i32, Direction) {
        let tile = &self.cells[r as usize][c as usize];

        let heading = match (tile, &facing) {
            // keeps on going
            (Tile::NS, Direction::South) => Direction::South,
            (Tile::NS, Direction::North) => Direction::North,
            (Tile::WE, Direction::West) => Direction::West,
            (Tile::WE, Direction::East) => Direction::East,

            // apporaches from other direction, turns
            (Tile::NE, Direction::South) => Direction::East,
            (Tile::NE, Direction::West) => Direction::North,

            (Tile::NW, Direction::South) => Direction::West,
            (Tile::NW, Direction::East) => Direction::North,

            (Tile::SE, Direction::North) => Direction::East,
            (Tile::SE, Direction::West) => Direction::South,

            (Tile::SW, Direction::North) => Direction::West,
            (Tile::SW, Direction::East) => Direction::South,

            _ => {
                unreachable!("missed {:?} {:?}", tile, facing)
            }
        };

        // return our new position and facing
        match heading {
            Direction::North => (r - 1, c, heading),
            Direction::South => (r + 1, c, heading),
            Direction::East => (r, c + 1, heading),
            Direction::West => (r, c - 1, heading),
        }
    }
}

impl Maze {
    fn looping_path(&self) -> Vec<(i32, i32, Direction)> {
        let direction = match self.get(self.start.0, self.start.1) {
            Tile::NS => Direction::North,
            Tile::NE => Direction::West,
            Tile::NW => Direction::East,
            Tile::WE => Direction::East,
            Tile::SW => Direction::North,
            Tile::SE => Direction::North,
            tile => unreachable!("start tile shouldn't be a {:?}", tile),
        };
        let Some(path) = bfs_loop(&(self.start.0, self.start.1, direction), |n| {
            vec![self.next_step(n.0, n.1, n.2)]
        }) else {
            unreachable!("there should be a looping path");
        };

        path
    }

    fn furthest_loop_distance(&self) -> usize {
        let path = self.looping_path();
        path.len() / 2
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = Maze::new(input);
    Some(maze.furthest_loop_distance())
}

impl Maze {
    fn contained_cells(&self) -> usize {
        0
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze = Maze::new(input);
    Some(maze.contained_cells())
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
    fn test_part_two_first() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_second() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_third() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_fourth() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(10));
    }
}
