use itertools::Itertools;

advent_of_code::solution!(9);

fn next_number(s: &str) -> i32 {
    let values: Vec<i32> = s
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect_vec();

    fn inner(values: Vec<i32>) -> i32 {
        let diff: Vec<i32> = values
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();

        if diff.iter().all(|&v| v == 0) {
            return *values.last().unwrap();
        }
        *values.last().unwrap() + inner(diff)
    }

    inner(values)
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.lines().map(next_number).sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;
    use test_case::test_case;

    #[test_case("0 3 6 9 12 15", 18)]
    fn test_next_number(s: &str, n: i32) {
        check!(next_number(s) == n);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
