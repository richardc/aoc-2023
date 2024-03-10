use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(12);

fn possible_completions<'a, 'b>(
    springs: &'a [u8],
    summary: &'b [usize],
    cache: &mut HashMap<(&'a [u8], &'b [usize]), usize>,
) -> usize {
    let cache_key = (springs, summary);
    if let Some(&result) = cache.get(&cache_key) {
        return result;
    }

    if summary.is_empty() {
        // We're out of groups to match, so this will be one match if
        // none of the remaining springs are clearly failed.
        let any_failed = springs.iter().any(|&b| b == b'#');
        let result = if any_failed { 0 } else { 1 };

        cache.insert(cache_key, result);
        return result;
    }

    let needed_space = summary.iter().sum::<usize>() + summary.len() - 1;
    if springs.len() < needed_space {
        // The summary specifies more springs than we have, so no match.
        cache.insert(cache_key, 0);
        return 0;
    }

    if springs[0] == b'.' {
        // Starting with success is about the same as starting with failure, but
        // less instructive, we can just skip
        return possible_completions(&springs[1..], summary, cache);
    }

    // Check the current group
    let group = summary[0];
    // All the springs must be failed or unknown
    let all_non_operational = springs[..group].iter().all(|&b| b != b'.');
    let end = (group + 1).min(springs.len());

    let mut solutions = 0;

    // If all the current group are failed or unknown -
    // and: There is a next spring and it's not failed
    // or: We're at the end of the spring list
    if all_non_operational
        && ((springs.len() > group && springs[group] != b'#') || (springs.len() <= group))
    {
        // We can recurse and testing the next group of springs
        solutions += possible_completions(&springs[end..], &summary[1..], cache);
    }

    // Starting with unknown, we can recurse and scan again just after
    if springs[0] == b'?' {
        solutions += possible_completions(&springs[1..], summary, cache)
    }

    cache.insert(cache_key, solutions);
    solutions
}

fn num_completions(s: &str) -> usize {
    let Some((record, summary)) = s.split_once(' ') else {
        unreachable!("no space in line");
    };
    let summary: Vec<usize> = summary.split(',').filter_map(|v| v.parse().ok()).collect();

    possible_completions(record.as_bytes(), &summary, &mut HashMap::new())
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

    #[test_case("? " => 1 ; "Trailing unknown")]
    #[test_case(". " => 1; "Trailing good")]
    #[test_case(".?.. " => 1; "Trailing mix of unknown and good")]
    #[test_case(".# " => 0; "Trailing failed")]
    #[test_case("??? 10" => 0 ; "Insufficent springs left")]
    #[test_case(". 1" => 0)]
    #[test_case("# 1" => 1)]
    #[test_case(".F 1" => 1)]
    fn test_num_completions(s: &str) -> usize {
        num_completions(s)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_unfold() {
        assert_eq!(unfold(".# 1"), ".#?.#?.#?.#?.# 1,1,1,1,1")
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
