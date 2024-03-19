use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(25);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node([u8; 3]);

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.0[0] as char, self.0[1] as char, self.0[2] as char
        )
    }
}

impl Node {
    fn new(s: &str) -> Self {
        Self(s.bytes().collect_vec().try_into().expect("3 bytes"))
    }
}

type Graph = HashMap<Node, HashSet<Node>>;

fn load(s: &str) -> Graph {
    let mut g: Graph = Default::default();
    for line in s.lines() {
        let (node, adj) = line.split_once(": ").unwrap();
        let node = Node::new(node);
        for other in adj.split(' ').map(Node::new) {
            g.entry(node)
                .and_modify(|e| {
                    e.insert(other);
                })
                .or_insert_with(|| HashSet::from_iter([other]));
            g.entry(other)
                .and_modify(|e| {
                    e.insert(node);
                })
                .or_insert_with(|| HashSet::from_iter([node]));
        }
    }

    g
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = load(input);

    dbg!(&graph);
    let mut left: HashSet<Node> = HashSet::from_iter(graph.keys().copied());
    let count = |node| graph.get(node).unwrap().difference(&left).count();
    while left.iter().map(count).sum::<usize>() != 3 {
        let max = left
            .iter()
            .max_by_key(|n| count(n))
            .expect("should have a max to remove");
        left.remove(&max);
    }

    Some(
        left.len()
            * HashSet::from_iter(graph.keys().copied())
                .difference(&left)
                .count(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(56));
    }
}
