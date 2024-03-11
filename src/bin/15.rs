advent_of_code::solution!(15);

fn hoho_hash(s: &[u8]) -> u32 {
    let mut acc: u32 = 0;
    for &b in s {
        acc += b as u32;
        acc *= 17;
        acc %= 256;
    }
    acc
}

fn hash_instructions(s: &str) -> u32 {
    s.trim().split(',').map(|c| hoho_hash(c.as_bytes())).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(hash_instructions(input))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hoho_hash() {
        let result = hoho_hash(b"HASH");
        assert_eq!(result, 52);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
