advent_of_code::solution!(23);

use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
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

#[derive(Default, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn neighbours<'a>(
        &'a self,
        maze: &'a Matrix<Cell>,
        slippy: bool,
    ) -> impl Iterator<Item = Pos> + 'a {
        use Cell::*;
        use Direction::*;
        maze.neighbours((self.0, self.1), false)
            .filter(move |&next| match maze.get(next).unwrap() {
                Forest => false,
                Slope(d) if slippy => !matches!(
                    (
                        (
                            self.0 as isize - next.0 as isize,
                            self.1 as isize - next.1 as isize,
                        ),
                        d,
                    ),
                    ((-1, 0), North) | ((0, 1), East) | ((1, 0), South) | ((0, -1), West),
                ),
                _ => true,
            })
            .map(|next| Pos(next.0, next.1))
    }
}

#[derive(Default, Debug)]
struct Graph(HashMap<Pos, HashMap<Pos, usize>>);

impl Graph {
    fn new(maze: &Matrix<Cell>, slippy: bool) -> Self {
        let Graph(mut graph) = Default::default();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from_iter([Pos(0, 1)]);

        while let Some(current) = queue.pop_front() {
            for neighbour in current.neighbours(maze, slippy) {
                let e = graph.entry(current).or_default();
                e.insert(neighbour, 1);

                if visited.insert(neighbour) {
                    graph.entry(neighbour).or_default();
                    queue.push_back(neighbour);
                }
            }
        }

        Graph(graph)
    }

    fn collapse(&self) -> Self {
        let mut graph = self.0.clone();

        // edge contraction
        let collapsable: Vec<Pos> = graph
            .iter()
            .filter_map(|(&node, edges)| if edges.len() == 2 { Some(node) } else { None })
            .collect_vec();

        for current in collapsable {
            let edges = graph.remove(&current).unwrap();
            let mut edges = edges.iter();
            let (left, left_distance) = edges.next().unwrap();
            let (right, right_distance) = edges.next().unwrap();

            let left_edges = graph.get_mut(left).unwrap();
            if left_edges.contains_key(&current) {
                let old = left_edges.remove(&current).unwrap();
                left_edges.insert(*right, old + right_distance);
            }

            let right_edges = graph.get_mut(right).unwrap();
            if right_edges.contains_key(&current) {
                let old = right_edges.remove(&current).unwrap();
                right_edges.insert(*left, old + left_distance);
            }
        }

        Graph(graph)
    }
}

fn load(input: &str) -> Matrix<Cell> {
    Matrix::from_iter(input.lines().map(|l| l.bytes().map(Cell::new)))
}

fn longest_path(graph: &Graph, start: Pos, goal: Pos) -> usize {
    fn walk(
        graph: &Graph,
        current: Pos,
        goal: Pos,
        visited: &mut HashSet<Pos>,
        distance: usize,
    ) -> usize {
        if current == goal {
            return distance;
        }

        visited.insert(current);

        let max = graph
            .0
            .get(&current)
            .unwrap()
            .iter()
            .filter_map(|(&neighbour, cost)| {
                if !visited.contains(&neighbour) {
                    Some(walk(graph, neighbour, goal, visited, distance + cost))
                } else {
                    None
                }
            })
            .max();

        visited.remove(&current);

        max.unwrap_or(0)
    }

    let mut visited: HashSet<Pos> = HashSet::new();

    walk(graph, start, goal, &mut visited, 0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = load(input);
    let graph = Graph::new(&maze, true);
    Some(longest_path(
        &graph,
        Pos(0, 1),
        Pos(maze.rows - 1, maze.columns - 2),
    ))
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze = load(input);
    let graph = Graph::new(&maze, false);
    let graph = graph.collapse();
    Some(longest_path(
        &graph,
        Pos(0, 1),
        Pos(maze.rows - 1, maze.columns - 2),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;
    use test_case::test_case;

    #[test_case(Pos(0,1) => vec![Pos(1,1)] ; "0,1: path south")]
    #[test_case(Pos(1,1) => vec![Pos(0,1), Pos(1,2)] ; "1,1: paths east and north")]
    #[test_case(Pos(3,3) => vec![Pos(3,4), Pos(4,3)]; "3,3: path east, slope south")]
    #[test_case(Pos(3,11) => vec![Pos(3,12), Pos(4,11)] ; "3,11: slopes east, south, west (impassable)")]
    #[test_case(Pos(5,3) => vec![Pos(5,4), Pos(6,3)] ; "5,3: slopes east, south, north (impassable)")]
    fn test_slippy_neighbours(start: Pos) -> Vec<Pos> {
        let maze = load(&advent_of_code::template::read_file("examples", DAY));
        start.neighbours(&maze, true).collect()
    }

    #[test_case(Pos(0,1) => vec![Pos(1,1)] ; "0,1: path south")]
    #[test_case(Pos(1,1) => vec![Pos(0,1), Pos(1,2)] ; "1,1: paths east and north")]
    #[test_case(Pos(3,3) => vec![Pos(3,4), Pos(4,3)]; "3,3: path east, slope south")]
    #[test_case(Pos(3,11) => vec![Pos(3,10), Pos(3,12), Pos(4,11)] ; "3,11: slopes west, east, south")]
    #[test_case(Pos(5,3) => vec![Pos(4,3), Pos(5,4), Pos(6,3)] ; "5,3: slopes north, east, south")]
    fn test_dry_neighbours(start: Pos) -> Vec<Pos> {
        let maze = load(&advent_of_code::template::read_file("examples", DAY));
        start.neighbours(&maze, false).collect()
    }

    #[test]
    fn test_graph_collapse() {
        let graph = Graph::new(
            &load(&advent_of_code::template::read_file_part(
                "examples", DAY, 2,
            )),
            false,
        );
        check!(
            graph.0
                == HashMap::from([
                    (Pos(0, 1), HashMap::from([(Pos(1, 1), 1)])),
                    (Pos(1, 1), HashMap::from([(Pos(0, 1), 1), (Pos(2, 1), 1)])),
                    (Pos(2, 1), HashMap::from([(Pos(1, 1), 1), (Pos(3, 1), 1)])),
                    (Pos(3, 1), HashMap::from([(Pos(2, 1), 1)])),
                ]),
        );

        check!(
            graph.collapse().0
                == HashMap::from([
                    (Pos(0, 1), HashMap::from([(Pos(3, 1), 3)])),
                    (Pos(3, 1), HashMap::from([(Pos(0, 1), 3)])),
                ]),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
