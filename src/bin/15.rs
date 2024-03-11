advent_of_code::solution!(15);

fn hoho_hash(s: &str) -> usize {
    let mut acc: usize = 0;
    for b in s.bytes() {
        acc += b as usize;
        acc *= 17;
        acc %= 256;
    }
    acc
}

fn hash_instructions(s: &str) -> usize {
    s.trim().split(',').map(hoho_hash).sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(hash_instructions(input))
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    length: usize,
}

fn focusing_power(s: &str) -> usize {
    const SLOTS: usize = 256;
    let mut boxes: [Vec<Lens>; SLOTS] = [(); SLOTS].map(|()| Vec::new());

    for instruction in s.trim().split(',') {
        if let Some((label, length)) = instruction.split_once('=') {
            let hash = hoho_hash(&label);
            let length = length.parse().unwrap();
            if let Some(index) = boxes[hash].iter().position(|l| l.label == label) {
                boxes[hash][index].length = length;
            } else {
                boxes[hash].push(Lens { label, length });
            }
        } else {
            let label = &instruction[..instruction.len() - 1];
            let hash = hoho_hash(label);
            if let Some(index) = boxes[hash].iter().position(|l| l.label == label) {
                boxes[hash].remove(index);
            }
        }
    }

    let mut power = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, lens) in b.iter().enumerate() {
            power += (i + 1) * (j + 1) * lens.length;
        }
    }
    power
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(focusing_power(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hoho_hash() {
        let result = hoho_hash("HASH");
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
        assert_eq!(result, Some(145));
    }
}
