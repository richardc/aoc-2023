use itertools::Itertools;

advent_of_code::solution!(11);

struct Image {
    data: Vec<Vec<bool>>,
    slow_rows: Vec<usize>,
    slow_cols: Vec<usize>,
}

impl Image {
    fn new(s: &str, slow: usize) -> Self {
        let data: Vec<Vec<bool>> = s
            .lines()
            .map(|l| l.bytes().map(|b| b == b'#').collect())
            .collect();
        let slow_rows = data
            .iter()
            .map(|row| if row.iter().all(|b| !*b) { slow } else { 0 })
            .collect();
        let slow_cols = (0..(data[0].len()))
            .map(|c| if data.iter().all(|r| !r[c]) { slow } else { 0 })
            .collect();
        Self {
            data,
            slow_rows,
            slow_cols,
        }
    }

    fn galaxies(&self) -> Vec<(usize, usize)> {
        self.data
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
            .map(|(&(r1, c1), &(r2, c2))| {
                let (r1, r2) = if r1 > r2 { (r2, r1) } else { (r1, r2) };
                let (c1, c2) = if c1 > c2 { (c2, c1) } else { (c1, c2) };
                self.slow_cols[c1..c2].iter().sum::<usize>()
                    + self.slow_rows[r1..r2].iter().sum::<usize>()
                    + (r2 - r1)
                    + (c2 - c1)
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let image = Image::new(input, 1);
    Some(image.sum_paths())
}

pub fn part_two(input: &str) -> Option<usize> {
    let image = Image::new(input, 1_000_000 - 1);
    Some(image.sum_paths())
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
    fn test_part_two_ten() {
        let image = Image::new(
            &advent_of_code::template::read_file("examples", DAY),
            10 - 1,
        );
        assert_eq!(image.sum_paths(), 1030);
    }

    #[test]
    fn test_part_two_hundred() {
        let image = Image::new(
            &advent_of_code::template::read_file("examples", DAY),
            100 - 1,
        );
        assert_eq!(image.sum_paths(), 8410);
    }
}
