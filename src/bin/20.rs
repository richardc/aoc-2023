use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Module {
    Broadcast,
    FlipFlop,
    Conjunction,
}

type Node<'a> = &'a str;

#[derive(Debug, Default)]
struct Network<'a> {
    connections: HashMap<Node<'a>, Vec<Node<'a>>>,
    modules: HashMap<Node<'a>, Module>,
    flipflops: HashMap<Node<'a>, bool>,
    conjunctions: HashMap<Node<'a>, HashMap<Node<'a>, Pulse>>,
    low: usize,
    high: usize,
}

impl<'b> Network<'b> {
    fn new<'a>(s: &'a str) -> Network<'a>
    where
        'b: 'a,
        'a: 'b,
    {
        let connections: HashMap<_, Vec<_>> = s
            .lines()
            .map(|l| {
                let (source, targets) = l.split_once(" -> ").unwrap();
                let source = source.trim_start_matches('&').trim_start_matches('%');
                (source, targets.split(", ").collect())
            })
            .collect();

        let modules: HashMap<_, _> = s
            .lines()
            .map(|l| {
                let (source, _) = l.split_once(' ').unwrap();
                match source.as_bytes()[0] {
                    b'%' => (&source[1..], Module::FlipFlop),
                    b'&' => (&source[1..], Module::Conjunction),
                    _ => (source, Module::Broadcast),
                }
            })
            .collect();

        let conjunctions: HashMap<_, _> = modules
            .iter()
            .filter_map(|(&k, &v)| {
                if v == Module::Conjunction {
                    Some((
                        k,
                        connections
                            .iter()
                            .filter_map(|(&from, to)| {
                                if to.contains(&k) {
                                    Some((from, Pulse::Low))
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    ))
                } else {
                    None
                }
            })
            .collect();

        Self {
            connections,
            modules,
            conjunctions,
            ..Default::default()
        }
    }

    fn send<'a>(&mut self, target: Node<'a>, pulse: Pulse)
    where
        'a: 'b,
        'b: 'a,
    {
        let mut queue: VecDeque<(Node, Node, Pulse)> = VecDeque::new();
        queue.push_back((target, "", pulse));

        while let Some((current, source, pulse)) = queue.pop_front() {
            let module = self.modules.get_mut(current);

            let mut send = pulse;

            match pulse {
                Pulse::Low => self.low += 1,
                Pulse::High => self.high += 1,
            };

            match (&module, pulse) {
                (Some(Module::FlipFlop), Pulse::Low) => {
                    let state = self.flipflops.entry(current).or_default();
                    *state = !*state;
                    send = if *state { Pulse::High } else { Pulse::Low };
                }
                (Some(Module::FlipFlop), _) => continue,
                (Some(Module::Conjunction), _) => {
                    let memory = self.conjunctions.get_mut(current).unwrap();
                    memory.insert(source, pulse);
                    send = if memory.values().all(|&v| v == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                }
                (Some(Module::Broadcast), _) => (),
                (None, _) => {
                    continue;
                }
            }

            for neighbour in self.connections.get(current).unwrap() {
                queue.push_back((neighbour, current, send));
            }
        }
    }

    fn pulse_count(&self) -> usize {
        self.low * self.high
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut network = Network::new(input);
    for _ in 0..1000 {
        network.send("broadcaster", Pulse::Low);
    }
    Some(network.pulse_count())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_first_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32_000_000));
    }

    #[test]
    fn test_part_one_second_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11_687_500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
