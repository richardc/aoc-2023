advent_of_code::solution!(13);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn new(b: u8) -> Self {
        match b {
            b'.' => Self::Ash,
            b'#' => Self::Rock,
            _ => unreachable!(),
        }
    }
}
struct Image {
    data: Vec<Vec<Tile>>,
}

#[derive(PartialEq, Debug)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Image {
    fn new(s: &str) -> Self {
        let data = s
            .lines()
            .map(|l| l.bytes().map(Tile::new).collect())
            .collect();
        Self { data }
    }

    fn reflection(&self) -> Reflection {
        if let Some(h) = self.horizontal() {
            return Reflection::Horizontal(h);
        }
        if let Some(v) = self.vertical() {
            return Reflection::Vertical(v);
        }
        unreachable!("image should have a reflection")
    }

    fn reflection_value(&self) -> usize {
        match self.reflection() {
            Reflection::Horizontal(h) => h * 100,
            Reflection::Vertical(v) => v,
        }
    }

    fn is_horizontal_mirror(&self, row: usize) -> bool {
        let above = row..self.data.len();
        let below = (0..row).rev();
        below
            .zip(above)
            .all(|(above, below)| self.data[above] == self.data[below])
    }

    fn horizontal(&self) -> Option<usize> {
        for i in 1..self.data.len() {
            if self.is_horizontal_mirror(i) {
                return Some(i);
            }
        }
        None
    }

    fn is_vertical_mirror(&self, col: usize) -> bool {
        let left = (0..col).rev();
        let right = col..self.data[0].len();
        left.zip(right).all(|(left, right)| {
            let left_col: Vec<Tile> = self.data.iter().map(|r| r[left]).collect();
            let right_col: Vec<Tile> = self.data.iter().map(|r| r[right]).collect();
            left_col == right_col
        })
    }

    fn vertical(&self) -> Option<usize> {
        for i in 1..self.data[0].len() {
            if self.is_vertical_mirror(i) {
                return Some(i);
            }
        }
        None
    }
}

fn load(i: &str) -> Vec<Image> {
    i.split("\n\n").map(Image::new).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let images = load(input);
    Some(images.iter().map(|i| i.reflection_value()).sum())
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_reflection_image0() {
        let images = load(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(images[0].reflection(), Reflection::Vertical(5));
    }

    #[test]
    fn test_reflection_image1() {
        let images = load(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(images[1].reflection(), Reflection::Horizontal(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
