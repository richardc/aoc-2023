advent_of_code::solution!(23);

use std::collections::HashSet;

use pathfinding::matrix::Matrix;

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

fn find_paths(maze: &Matrix<Cell>, start: Pos, goal: Pos) -> Vec<Vec<Pos>> {
    fn walk(
        maze: &Matrix<Cell>,
        current: Pos,
        goal: Pos,
        visited: &mut HashSet<Pos>,
        current_path: &mut Vec<Pos>,
        all_paths: &mut Vec<Vec<Pos>>,
    ) {
        if !visited.insert(current) {
            return;
        }

        current_path.push(current);

        if current == goal {
            all_paths.push(current_path.clone());
            visited.remove(&current);
            current_path.pop();
            return;
        }

        for neighbour in current.neighbours(maze) {
            walk(maze, neighbour, goal, visited, current_path, all_paths);
        }

        current_path.pop();
        visited.remove(&current);
    }

    let mut visited: HashSet<Pos> = HashSet::new();
    let mut current: Vec<Pos> = Vec::new();
    let mut all: Vec<Vec<Pos>> = Vec::new();

    walk(maze, start, goal, &mut visited, &mut current, &mut all);

    all
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = load(input);
    let paths = find_paths(&maze, Pos(0, 1), Pos(maze.rows - 1, maze.columns - 2));

    paths.iter().map(|p| p.len() - 1).max()
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
