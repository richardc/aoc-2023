use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Debug, PartialEq, Clone)]
struct Line {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    a: f64,
    b: f64,
}

impl Line {
    fn new(s: &str) -> Self {
        let (point, vector) = s.split_once('@').unwrap();
        let (x, y, _z) = point
            .split(',')
            .map(|c| c.trim().parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (vx, vy, _vz) = vector
            .split(',')
            .map(|c| c.trim().parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();

        // rearrange into form y = ax + b
        let a = vy / vx;
        let b = y - a * x;
        Self { x, y, vx, vy, a, b }
    }

    fn intersect_2d(&self, other: &Self) -> Option<(f64, f64)> {
        if self.a == other.a {
            // paralell
            return None;
        }

        let x = (other.b - self.b) / (self.a - other.a);
        let y = self.a * x + self.b;
        Some((x, y))
    }

    fn in_future(&self, (x, y): (f64, f64)) -> bool {
        if self.vx < 0.0 && self.x < x {
            return false;
        }
        if self.vx > 0.0 && self.x > x {
            return false;
        }
        if self.vy < 0.0 && self.y < y {
            return false;
        }
        if self.vy > 0.0 && self.y > y {
            return false;
        }
        true
    }

    fn intersect_in_future(&self, other: &Self) -> Option<(f64, f64)> {
        if let Some(intersects) = self.intersect_2d(other) {
            if self.in_future(intersects) && other.in_future(intersects) {
                return Some(intersects);
            }
        }
        None
    }
}

fn crossing_in_zone(input: &str, min: f64, max: f64) -> usize {
    input
        .lines()
        .map(Line::new)
        .combinations(2)
        .filter_map(|pair| pair[0].intersect_in_future(&pair[1]))
        .filter(|&(x, y)| x >= min && x <= max && y >= min && y <= max)
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(crossing_in_zone(
        input,
        200000000000000.0,
        400000000000000.0,
    ))
}

pub fn part_two(input: &str) -> Option<u32> {
    let _ = input;
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0, 0, 30 @ 1, 1, -2" => Line { x: 0.0, y:0.0, vx: 1.0, vy: 1.0, a: 1.0, b: 0.0 } ; "0,0 -> 1,1")]
    #[test_case("0, 1, 30 @ 1, 1, -2" => Line { x: 0.0, y:1.0, vx: 1.0, vy: 1.0,  a: 1.0, b: 1.0 } ; "0,1 -> 1,2")]
    #[test_case("0, 0, 30 @ -1, -1, -2" => Line { x: 0.0, y: 0.0, vx: -1.0, vy: -1.0, a: 1.0, b: 0.0 } ; "0,1 -> 0,0")]
    #[test_case("19, 13, 30 @ -2, 1, -2" => Line { x: 19.0, y: 13.0, vx: -2.0, vy: 1.0, a: -0.5, b: 22.5 } ; "First example")]
    fn test_line_new(s: &str) -> Line {
        Line::new(s)
    }

    #[test]
    fn test_line_intersect() {
        let a = Line::new("19, 13, 30 @ -2, 1, -2");
        let b = Line::new("12, 31, 28 @ -1, -2, -1");
        let intersects = a.intersect_2d(&b);
        assert_eq!(intersects, Some((6.2, 19.4)));
    }

    #[test]
    fn test_line_in_future() {
        let a = Line::new("20, 25, 34 @ -2, -2, -4");
        let b = Line::new("20, 19, 15 @ 1, -5, -3");

        let intersects = a.intersect_2d(&b);
        assert_eq!(intersects, Some((19.0, 24.0)));

        assert!(a.in_future(intersects.unwrap()));
        assert!(!b.in_future(intersects.unwrap()));
    }

    #[test]
    fn test_part_one() {
        let result = crossing_in_zone(
            &advent_of_code::template::read_file("examples", DAY),
            7.0,
            27.0,
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
