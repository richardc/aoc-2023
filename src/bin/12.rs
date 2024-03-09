use itertools::Itertools;

advent_of_code::solution!(12);

fn matches_record(s: &str, record: &[usize]) -> bool {
    let pattern: Vec<usize> = s
        .bytes()
        .group_by(|&b| b == b'#')
        .into_iter()
        .filter_map(|(s, g)| if s { Some(g.count()) } else { None })
        .collect();
    record == pattern
}

fn generate_records(p: &str) -> Vec<String> {
    if let Some((prefix, rest)) = p.split_once('?') {
        return generate_records(rest)
            .iter()
            .flat_map(|t| vec![prefix.to_owned() + "." + t, prefix.to_owned() + "#" + t])
            .collect();
    }
    vec![p.to_string()]
}

fn num_completions(s: &str) -> usize {
    let Some((record, summary)) = s.split_once(' ') else {
        unreachable!("no space in line");
    };
    let summary: Vec<usize> = summary.split(',').map(|v| v.parse().unwrap()).collect();

    generate_records(record)
        .iter()
        .filter(|r| matches_record(r, &summary))
        .count()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().map(num_completions).sum())
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("#.#", &[1,1])]
    fn test_matches(s: &str, r: &[usize]) {
        assert!(matches_record(s, r))
    }

    #[test]
    fn test_generates_single() {
        assert_eq!(generate_records("?"), vec![".", "#"])
    }

    #[test]
    fn test_generates_double() {
        assert_eq!(generate_records("?.?"), vec!["...", "#..", "..#", "#.#"])
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
