use itertools::Itertools;

advent_of_code::solution!(11);

struct Image {
    galaxies: Vec<Vec<bool>>,
}

impl Image {
    fn new(s: &str) -> Self {
        Self {
            galaxies: s
                .lines()
                .map(|l| l.bytes().map(|b| b == b'#').collect())
                .collect(),
        }
    }

    fn galaxies(&self) -> Vec<(usize, usize)> {
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(c, b)| if *b { Some((r, c)) } else { None })
            })
            .collect()
    }

    fn sum_paths(&self) -> usize {
        self.galaxies()
            .iter()
            .tuple_combinations()
            .map(|(&(y1, x1), &(y2, x2))| y1.abs_diff(y2) + x1.abs_diff(x2))
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let image = Image::new(input);
    Some(image.sum_paths())
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
