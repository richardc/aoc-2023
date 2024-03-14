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

    fn send<'a, FN>(&mut self, target: Node<'a>, pulse: Pulse, mut peek: FN)
    where
        FN: FnMut(Node<'a>, Node<'a>, Pulse),
        'a: 'b,
        'b: 'a,
    {
        let mut queue: VecDeque<(Node, Node, Pulse)> = VecDeque::new();
        queue.push_back((target, "", pulse));

        while let Some((current, source, pulse)) = queue.pop_front() {
            peek(current, source, pulse);
            let module = self.modules.get_mut(current);
            let mut send = pulse;

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

    fn pulse_count(&mut self) -> usize {
        let mut low = 0;
        let mut high = 0;

        for _ in 0..1000 {
            self.send("broadcaster", Pulse::Low, |_, _, pulse| match pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            });
        }
        low * high
    }

    fn when_rx_goes_low(&mut self) -> usize {
        let rx_sources: Vec<_> = self
            .connections
            .iter()
            .filter_map(|(r, v)| if v.contains(&"rx") { Some(r) } else { None })
            .collect();
        assert!(rx_sources.len() == 1);
        let rx_source = *rx_sources[0];
        assert!(self.modules.get(rx_source) == Some(&Module::Conjunction));

        let mut steps: HashMap<&str, usize> = self
            .connections
            .iter()
            .filter_map(|(&r, v)| {
                if v.contains(&rx_source) {
                    Some((r, 0))
                } else {
                    None
                }
            })
            .collect();
        assert!(steps
            .keys()
            .all(|source| self.modules.get(source) == Some(&Module::Conjunction)));

        let mut step = 0;
        loop {
            step += 1;
            self.send("broadcaster", Pulse::Low, |to, from, pulse| {
                if to == rx_source && pulse == Pulse::High {
                    // This is the cycle for the module `from`
                    steps.entry(from).and_modify(|s| *s = step);
                }
            });

            // We have all cycles, time to maths
            if steps.values().all(|&v| v > 0) {
                break;
            }
        }
        steps.values().map(|&v| v).reduce(num_integer::lcm).unwrap()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(Network::new(input).pulse_count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Network::new(input).when_rx_goes_low())
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
}
