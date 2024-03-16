advent_of_code::solution!(22);

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Default, PartialOrd, PartialEq, Eq)]
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

impl Ord for Point3D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z.cmp(&other.z)
    }
}

type BrickId = usize;

#[derive(Debug, Default, PartialOrd, Ord, PartialEq, Eq)]
struct Brick {
    id: BrickId,
    bounds: (Point3D, Point3D),
}

impl Brick {
    fn new(s: &str, id: BrickId) -> Self {
        let bounds = s
            .split('~')
            .map(Point3D::new)
            .sorted()
            .collect_tuple()
            .expect("pair of points");

        Self { id, bounds }
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

#[derive(Debug, Default)]
struct Pile {
    bricks: HashMap<BrickId, Brick>,
    supports: HashMap<BrickId, Vec<BrickId>>,
    rests_on: HashMap<BrickId, Vec<BrickId>>,
}

impl Pile {
    fn new(s: &str) -> Self {
        let bricks = s
            .lines()
            .enumerate()
            .map(|(i, l)| (i, Brick::new(l, i)))
            .collect();
        Self {
            bricks,
            ..Default::default()
        }
    }

    fn drop(&mut self) {
        let mut heights: HashMap<(i32, i32), i32> = HashMap::new();
        let mut occupies: HashMap<(i32, i32), BrickId> = HashMap::new();
        for brick in self.bricks.values().sorted() {
            let base = *brick
                .covers()
                .iter()
                .filter_map(|point| heights.get(point))
                .max()
                .unwrap_or(&0);

            let top = base + brick.height();
            let mut lands_on: HashSet<BrickId> = HashSet::new();
            for (x, y) in brick.covers() {
                if let Some(height) = heights.insert((x, y), top) {
                    if height == base {
                        lands_on.insert(*occupies.get(&(x, y)).unwrap());
                    }
                }
                occupies.insert((x, y), brick.id);
            }

            for other in &lands_on {
                self.supports
                    .entry(*other)
                    .and_modify(|v| v.push(brick.id))
                    .or_insert_with(|| vec![brick.id]);
            }

            self.rests_on
                .insert(brick.id, lands_on.iter().copied().collect());
        }
    }

    fn safely_removable(&self) -> usize {
        self.bricks
            .keys()
            .filter(|b| {
                if let Some(supporting) = self.supports.get(b) {
                    supporting
                        .iter()
                        .all(|s| self.rests_on.get(s).unwrap().len() > 1)
                } else {
                    true
                }
            })
            .count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut pile = Pile::new(input);
    pile.drop();

    Some(pile.safely_removable())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brick_new() {
        let result = Brick::new("2,2,2~2,2,2", 0);
        assert_eq!(
            result,
            Brick {
                bounds: (Point3D { x: 2, y: 2, z: 2 }, Point3D { x: 2, y: 2, z: 2 }),
                id: 0,
            },
        )
    }

    #[test]
    fn test_brick_height() {
        let result = Brick::new("2,2,2~2,2,2", 0).height();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_brick_covers() {
        let result = Brick::new("2,2,2~2,2,2", 0).covers();
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
        assert_eq!(result, None);
    }
}
