advent_of_code::solution!(22);

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Default, PartialEq, Eq)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn new(s: &str) -> Self {
        let (x, y, z) = s
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .expect("parsing point");
        Self { x, y, z }
    }
}

impl PartialOrd for Point3D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point3D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z.cmp(&other.z)
    }
}

#[derive(Debug, Default, PartialEq)]
struct Brick {
    bounds: (Point3D, Point3D),
    supports: HashSet<usize>,
    supported_by: HashSet<usize>,
}

impl Brick {
    fn new(s: &str) -> Self {
        let bounds = s
            .split('~')
            .map(Point3D::new)
            .sorted()
            .collect_tuple()
            .expect("pair of points");

        Self {
            bounds,
            ..Default::default()
        }
    }

    fn covers(&self) -> Vec<(i32, i32)> {
        let (x1, x2) = if self.bounds.0.x < self.bounds.1.x {
            (self.bounds.0.x, self.bounds.1.x)
        } else {
            (self.bounds.1.x, self.bounds.0.x)
        };
        let (y1, y2) = if self.bounds.0.y < self.bounds.1.y {
            (self.bounds.0.y, self.bounds.1.y)
        } else {
            (self.bounds.1.y, self.bounds.0.y)
        };
        (x1..=x2).cartesian_product(y1..=y2).collect()
    }

    fn height(&self) -> i32 {
        self.bounds.1.z - self.bounds.0.z + 1
    }
}

#[derive(Debug)]
struct Pile {
    bricks: Vec<Brick>,
}

impl Pile {
    fn new(s: &str) -> Self {
        let bricks = s
            .lines()
            .map(Brick::new)
            .sorted_by(|a, b| a.bounds.cmp(&b.bounds))
            .collect();
        Self { bricks }
    }

    fn drop(&mut self) {
        let mut heights: HashMap<(i32, i32), i32> = HashMap::new();
        let mut occupies: HashMap<(i32, i32), usize> = HashMap::new();
        let mut supports: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (idx, brick) in self.bricks.iter_mut().enumerate() {
            let base = *brick
                .covers()
                .iter()
                .filter_map(|point| heights.get(point))
                .max()
                .unwrap_or(&0);

            let top = base + brick.height();
            for (x, y) in brick.covers() {
                if let Some(height) = heights.insert((x, y), top) {
                    if height == base {
                        let other = *occupies.get(&(x, y)).unwrap();

                        brick.supported_by.insert(other);
                        // Avoid mulitiple mutable borrows from:
                        //     self.bricks[other].supports.insert(idx);
                        // Build up another variable and zoop it across after the brick loop
                        supports
                            .entry(other)
                            .and_modify(|s| {
                                s.insert(idx);
                            })
                            .or_insert_with(|| HashSet::from([idx]));
                    }
                }
                occupies.insert((x, y), idx);
            }
        }

        for (idx, set) in supports {
            self.bricks[idx].supports.extend(set);
        }
    }

    fn safely_removable(&self) -> usize {
        self.bricks
            .iter()
            .filter(|brick| {
                brick
                    .supports
                    .iter()
                    .all(|other| self.bricks[*other].supported_by.len() > 1)
            })
            .count()
    }

    fn tumble_sum(&self) -> usize {
        let mut result = 0;

        for (idx, brick) in self.bricks.iter().enumerate() {
            if brick.supports.is_empty() {
                continue;
            }
            let mut fallen: HashSet<usize> = HashSet::new();
            let mut queue: VecDeque<usize> = VecDeque::new();

            queue.push_back(idx);

            while let Some(idx) = queue.pop_front() {
                fallen.insert(idx);

                for &supported in &self.bricks[idx].supports {
                    if self.bricks[supported]
                        .supported_by
                        .iter()
                        .all(|support| fallen.contains(support))
                    {
                        queue.push_back(supported);
                    }
                }
            }

            result += fallen.len() - 1;
        }
        result
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut pile = Pile::new(input);
    pile.drop();

    Some(pile.safely_removable())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut pile = Pile::new(input);
    pile.drop();

    Some(pile.tumble_sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brick_new() {
        let result = Brick::new("2,2,2~2,2,2");
        assert_eq!(
            result,
            Brick {
                bounds: (Point3D { x: 2, y: 2, z: 2 }, Point3D { x: 2, y: 2, z: 2 }),
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_brick_height() {
        let result = Brick::new("2,2,2~2,2,2").height();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_brick_covers() {
        let result = Brick::new("2,2,2~2,2,2").covers();
        assert_eq!(result, vec![(2, 2)]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
