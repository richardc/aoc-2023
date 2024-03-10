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
    smudge: bool,
}

#[derive(PartialEq, Debug)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

impl Image {
    fn new(s: &str, smudge: bool) -> Self {
        let data = s
            .lines()
            .map(|l| l.bytes().map(Tile::new).collect())
            .collect();
        Self { data, smudge }
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
        let mut matching = 0;
        let mut matches = 0;
        for (first, second) in below.zip(above) {
            for col in 0..self.data[0].len() {
                matches += 1;
                if self.data[first][col] == self.data[second][col] {
                    matching += 1;
                }
            }
        }
        if self.smudge {
            matching == matches - 1
        } else {
            matching == matches
        }
    }

    fn horizontal(&self) -> Option<usize> {
        (1..self.data.len()).find(|&i| self.is_horizontal_mirror(i))
    }

    fn is_vertical_mirror(&self, col: usize) -> bool {
        let left = (0..col).rev();
        let right = col..self.data[0].len();
        let mut matching = 0;
        let mut matches = 0;
        for (first, second) in left.zip(right) {
            for row in 0..self.data.len() {
                matches += 1;
                if self.data[row][first] == self.data[row][second] {
                    matching += 1;
                }
            }
        }

        if self.smudge {
            matching == matches - 1
        } else {
            matching == matches
        }
    }

    fn vertical(&self) -> Option<usize> {
        (1..self.data[0].len()).find(|&i| self.is_vertical_mirror(i))
    }
}

fn load(i: &str, smudge: bool) -> Vec<Image> {
    i.split("\n\n").map(|raw| Image::new(raw, smudge)).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let images = load(input, false);
    Some(images.iter().map(|i| i.reflection_value()).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let images = load(input, true);
    Some(images.iter().map(|i| i.reflection_value()).sum())
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
        let images = load(&advent_of_code::template::read_file("examples", DAY), false);
        assert_eq!(images[0].reflection(), Reflection::Vertical(5));
    }

    #[test]
    fn test_reflection_image1() {
        let images = load(&advent_of_code::template::read_file("examples", DAY), false);
        assert_eq!(images[1].reflection(), Reflection::Horizontal(4));
    }

    #[test]
    fn test_reflection_image0_smudged() {
        let images = load(&advent_of_code::template::read_file("examples", DAY), true);
        assert_eq!(images[0].reflection(), Reflection::Horizontal(3));
    }

    #[test]
    fn test_reflection_image1_smudged() {
        let images = load(&advent_of_code::template::read_file("examples", DAY), true);
        assert_eq!(images[1].reflection(), Reflection::Horizontal(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
