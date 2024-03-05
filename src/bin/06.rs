advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn winning_starts(&self) -> Vec<u32> {
        let mut starts = Vec::new();
        for start in 0..self.time {
            let travelled = (self.time - start) * start;
            if travelled > self.distance {
                starts.push(start);
            }
        }
        starts
    }
}

fn parse(s: &str) -> Vec<Race> {
    let Some((times, distances)) = s.split_once('\n') else {
        panic!("malformed data")
    };
    let Some((_, t)) = times.split_once(':') else {
        panic!(":");
    };
    let Some((_, d)) = distances.split_once(':') else {
        panic!(":");
    };

    t.trim()
        .split_ascii_whitespace()
        .zip(d.trim().split_ascii_whitespace())
        .map(|(a, b)| Race {
            time: a.parse().unwrap(),
            distance: b.parse().unwrap(),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse(input);
    Some(
        races
            .iter()
            .map(|r| r.winning_starts().len() as u32)
            .product(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_winning_starts() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(race.winning_starts(), vec![2, 3, 4, 5])
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
