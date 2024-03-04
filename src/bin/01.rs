advent_of_code::solution!(1);

pub fn extract_digits(line: &str) -> Vec<u8> {
    line.as_bytes()
        .iter()
        .filter_map(|b| {
            if b.is_ascii_digit() {
                Some(b - b'0')
            } else {
                None
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let digits = extract_digits(line);
                let first = digits.first().unwrap();
                let last = digits.last().unwrap();
                u32::from(first * 10 + last)
            })
            .sum(),
    )
}

pub fn extract_word_digits(line: &str) -> Vec<u8> {
    let mut digits: Vec<u8> = vec![];
    let mut b = line.as_bytes();
    while !b.is_empty() {
        match b {
            b if b.starts_with(b"one") => {
                digits.push(1);
            }
            b if b.starts_with(b"two") => {
                digits.push(2);
            }
            b if b.starts_with(b"three") => {
                digits.push(3);
            }
            b if b.starts_with(b"four") => {
                digits.push(4);
            }
            b if b.starts_with(b"five") => {
                digits.push(5);
            }
            b if b.starts_with(b"six") => {
                digits.push(6);
            }
            b if b.starts_with(b"seven") => {
                digits.push(7);
            }
            b if b.starts_with(b"eight") => {
                digits.push(8);
            }
            b if b.starts_with(b"nine") => {
                digits.push(9);
            }
            b if b[0].is_ascii_digit() => digits.push(b[0] - b'0'),
            _ => {}
        }
        b = &b[1..];
    }

    digits
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let digits = extract_word_digits(line);
                let first = digits.first().unwrap();
                let last = digits.last().unwrap();
                u32::from(first * 10 + last)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }

    #[test]
    fn test_extract_word_digits() {
        let result = extract_word_digits("oneight");
        assert_eq!(result, vec![1, 8]);
    }
}
