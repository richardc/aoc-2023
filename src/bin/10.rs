use pathfinding::directed::bfs::bfs_loop;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Empty,      // .
    Vertical,   // |
    Horizontal, // -
    CornerL,    // L
    CornerJ,    // J
    Corner7,    // 7,
    CornerF,    // F
    Start,      // S
}

impl Tile {
    fn new(c: u8) -> Self {
        match c {
            b'.' => Self::Empty,
            b'|' => Self::Vertical,
            b'-' => Self::Horizontal,
            b'L' => Self::CornerL,
            b'J' => Self::CornerJ,
            b'7' => Self::Corner7,
            b'F' => Self::CornerF,
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
            // Vertical tiles north and south connect Vertically
            (Tile::Vertical, Tile::Vertical, _, _) => Tile::Vertical,

            (Tile::CornerF, Tile::CornerL, _, _) => Tile::Vertical,
            (Tile::CornerF, Tile::CornerJ, _, _) => Tile::Vertical,

            (Tile::Corner7, Tile::CornerL, _, _) => Tile::Vertical,
            (Tile::Corner7, Tile::CornerJ, _, _) => Tile::Vertical,

            // Horizontal tiles west and east connect E W
            (_, _, Tile::Horizontal, Tile::Horizontal) => Tile::Horizontal,

            (_, _, Tile::CornerL, Tile::Horizontal) => Tile::Horizontal,
            (_, _, Tile::CornerF, Tile::Horizontal) => Tile::Horizontal,

            (_, _, Tile::Horizontal, Tile::CornerJ) => Tile::Horizontal,
            (_, _, Tile::Horizontal, Tile::Corner7) => Tile::Horizontal,

            // CornerJ tiles north and west connect S E
            (Tile::Vertical, _, Tile::Horizontal, _) => Tile::CornerJ,

            (Tile::Corner7, _, Tile::Horizontal, _) => Tile::CornerJ,
            (Tile::CornerF, _, Tile::Horizontal, _) => Tile::CornerJ,

            (Tile::Corner7, _, Tile::CornerL, _) => Tile::CornerJ,
            (Tile::CornerF, _, Tile::CornerF, _) => Tile::CornerJ,

            // CornerL tiles north and east connect S W
            (Tile::Vertical, _, _, Tile::Horizontal) => Tile::CornerL,

            (Tile::Corner7, _, _, Tile::Horizontal) => Tile::CornerL,
            (Tile::CornerF, _, _, Tile::Horizontal) => Tile::CornerL,

            (Tile::Corner7, _, _, Tile::CornerJ) => Tile::CornerL,
            (Tile::CornerF, _, _, Tile::Corner7) => Tile::CornerL,

            // Corner7 tiles south and east connect N E
            (_, Tile::Vertical, Tile::Horizontal, _) => Tile::Corner7,

            (_, Tile::CornerJ, Tile::Horizontal, _) => Tile::Corner7,
            (_, Tile::CornerL, Tile::Horizontal, _) => Tile::Corner7,

            (_, Tile::Vertical, Tile::CornerL, _) => Tile::Corner7,
            (_, Tile::Vertical, Tile::CornerF, _) => Tile::Corner7,

            // CornerF tiles south and west connect N W
            (_, Tile::Vertical, _, Tile::Horizontal) => Tile::CornerF,

            (_, Tile::CornerJ, _, Tile::Horizontal) => Tile::CornerF,
            (_, Tile::CornerL, _, Tile::Horizontal) => Tile::CornerF,

            (_, Tile::Vertical, _, Tile::CornerJ) => Tile::CornerF,
            (_, Tile::Vertical, _, Tile::Corner7) => Tile::CornerF,
            (_, Tile::CornerJ, _, Tile::Corner7) => Tile::CornerF,
            (_, Tile::CornerF, _, Tile::Corner7) => Tile::CornerF,

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
            (Tile::Vertical, Direction::South) => Direction::South,
            (Tile::Vertical, Direction::North) => Direction::North,
            (Tile::Horizontal, Direction::West) => Direction::West,
            (Tile::Horizontal, Direction::East) => Direction::East,

            // apporaches from other direction, turns
            (Tile::CornerL, Direction::South) => Direction::East,
            (Tile::CornerL, Direction::West) => Direction::North,

            (Tile::CornerJ, Direction::South) => Direction::West,
            (Tile::CornerJ, Direction::East) => Direction::North,

            (Tile::CornerF, Direction::North) => Direction::East,
            (Tile::CornerF, Direction::West) => Direction::South,

            (Tile::Corner7, Direction::North) => Direction::West,
            (Tile::Corner7, Direction::East) => Direction::South,

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
            Tile::Vertical => Direction::North,
            Tile::CornerL => Direction::West,
            Tile::CornerJ => Direction::East,
            Tile::Horizontal => Direction::East,
            Tile::Corner7 => Direction::North,
            Tile::CornerF => Direction::North,
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
        assert_eq!(maze.start_tile(), Tile::CornerF);
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
