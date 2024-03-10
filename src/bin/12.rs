use itertools::Itertools;

advent_of_code::solution!(12);

fn matches_record(s: &[u8], record: &[usize]) -> bool {
    let pattern: Vec<usize> = s
        .iter()
        .group_by(|&&b| b == b'#')
        .into_iter()
        .filter_map(|(s, g)| if s { Some(g.count()) } else { None })
        .collect();
    record == pattern
}

fn generate_records(pattern: &str) -> RecordIter {
    let pattern = pattern.bytes().collect_vec();
    let indexes = (0..pattern.len())
        .filter(|&i| pattern[i] == b'?')
        .collect_vec();
    let max = 1 << indexes.len();
    RecordIter {
        pattern,
        indexes,
        index: 0,
        max,
    }
}

struct RecordIter {
    pattern: Vec<u8>,
    indexes: Vec<usize>,
    index: usize,
    max: usize,
}

impl Iterator for RecordIter {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.max {
            let mut pattern = self.pattern.clone();
            for i in 0..self.indexes.len() {
                pattern[self.indexes[i]] = if self.index & (1 << i) != 0 {
                    b'#'
                } else {
                    b'.'
                };
            }
            self.index += 1;
            return Some(pattern);
        }
        None
    }
}

fn num_completions(s: &str) -> usize {
    let Some((record, summary)) = s.split_once(' ') else {
        unreachable!("no space in line");
    };
    let summary: Vec<usize> = summary.split(',').map(|v| v.parse().unwrap()).collect();

    generate_records(record)
        .filter(|r| matches_record(r, &summary))
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().map(num_completions).sum())
}

pub fn unfold(s: &str) -> String {
    let Some((record, summary)) = s.split_once(' ') else {
        unreachable!("no space in line");
    };

    (0..5).map(|_| record).join("?") + " " + &(0..5).map(|_| summary).join(",")
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(input.lines().map(unfold).map(|s| num_completions(&s)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(b"#.#", &[1,1])]
    fn test_matches(s: &[u8], r: &[usize]) {
        assert!(matches_record(s, r))
    }

    #[test]
    fn test_generates_single() {
        assert_eq!(
            generate_records("?")
                .map(|v| String::from_utf8(v.to_vec()).unwrap())
                .collect_vec(),
            vec![".", "#"]
        )
    }

    #[test]
    fn test_generates_double() {
        assert_eq!(
            generate_records("?.?")
                .map(|v| String::from_utf8(v.to_vec()).unwrap())
                .collect_vec(),
            vec!["...", "#..", "..#", "#.#"]
        )
    }

    #[test]
    fn test_unfold() {
        assert_eq!(unfold(".# 1"), ".#?.#?.#?.#?.# 1,1,1,1,1")
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
