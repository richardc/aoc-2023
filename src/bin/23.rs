advent_of_code::solution!(23);

use pathfinding::matrix::Matrix;
use pathfinding::prelude::edmonds_karp_dense;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Cell {
    Path,
    Forest,
    Slope(Direction),
}

impl Cell {
    fn new(b: u8) -> Self {
        use Cell::*;
        use Direction::*;
        match b {
            b'#' => Forest,
            b'.' => Path,
            b'^' => Slope(North),
            b'>' => Slope(East),
            b'v' => Slope(South),
            b'<' => Slope(West),
            _ => unreachable!("decode of {b}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn neighbours<'a>(&'a self, maze: &'a Matrix<Cell>) -> impl Iterator<Item = Pos> + 'a {
        use Cell::*;
        use Direction::*;
        maze.neighbours((self.0, self.1), false)
            .filter(|&next| match maze.get(next).unwrap() {
                Path => true,
                Forest => false,
                Slope(d) => match (
                    (
                        self.0 as isize - next.0 as isize,
                        self.1 as isize - next.1 as isize,
                    ),
                    d,
                ) {
                    ((-1, 0), North) => false,
                    ((0, 1), East) => false,
                    ((1, 0), South) => false,
                    ((0, -1), West) => false,
                    _ => true,
                },
            })
            .map(|next| Pos(next.0, next.1))
    }
}

fn load(input: &str) -> Matrix<Cell> {
    Matrix::from_iter(input.lines().map(|l| l.bytes().map(Cell::new)))
}

pub fn part_one(input: &str) -> Option<i32> {
    let maze = load(input);

    let vertices: Vec<_> = maze
        .items()
        .filter_map(|((r, c), cell)| {
            if matches!(cell, Cell::Path | Cell::Slope(_)) {
                Some(Pos(r, c))
            } else {
                None
            }
        })
        .collect();

    let capacities = vertices
        .iter()
        .flat_map(|p| p.neighbours(&maze).map(|n| ((*p, n), 1)));

    let (nodes, _, _) = edmonds_karp_dense(
        &vertices,
        &Pos(0, 1),
        &Pos(maze.rows - 1, maze.columns - 2),
        capacities,
    );

    Some(nodes.len() as i32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Pos(0,1) => vec![Pos(1,1)] ; "0,1: path south")]
    #[test_case(Pos(1,1) => vec![Pos(0,1), Pos(1,2)] ; "1,1: paths east and north")]
    #[test_case(Pos(3,3) => vec![Pos(3,4), Pos(4,3)]; "3,3: path east, slope south")]
    #[test_case(Pos(3,11) => vec![Pos(3,12), Pos(4,11)] ; "3,11: slopes east, south, west (impassable)")]
    #[test_case(Pos(5,3) => vec![Pos(5,4), Pos(6,3)] ; "5,3: slopes east, south, north (impassable)")]
    fn test_neighbours(start: Pos) -> Vec<Pos> {
        let maze = load(&advent_of_code::template::read_file("examples", DAY));
        start.neighbours(&maze).collect()
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
