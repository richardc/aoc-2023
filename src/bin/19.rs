use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(19);

type Value = u64;

enum Op {
    Gt,
    Lt,
}

impl Op {
    fn new(b: u8) -> Self {
        use Op::*;
        match b {
            b'>' => Gt,
            b'<' => Lt,
            _ => unreachable!("bad op {}", b as char),
        }
    }
}

#[derive(Clone, Copy)]
enum Field {
    X,
    M,
    A,
    S,
}

impl Field {
    fn new(b: u8) -> Self {
        use Field::*;
        match b {
            b'x' => X,
            b'm' => M,
            b'a' => A,
            b's' => S,
            _ => unreachable!("bad field {}", b as char),
        }
    }
}

struct Rule<'a> {
    field: Field,
    op: Op,
    value: Value,
    target: &'a str,
}

impl Rule<'_> {
    fn new(s: &str) -> Rule<'_> {
        let Some((expr, target)) = s.split_once(':') else {
            unreachable!("rule without :")
        };
        let field = Field::new(expr.as_bytes()[0]);
        let op = Op::new(expr.as_bytes()[1]);
        let value = expr[2..].parse().unwrap();
        Rule {
            field,
            op,
            value,
            target,
        }
    }

    fn evaluate(&self, p: &Product) -> Option<&str> {
        if match self.op {
            Op::Gt => p.get(self.field) > self.value,
            Op::Lt => p.get(self.field) < self.value,
        } {
            return Some(self.target);
        }
        None
    }
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    default: &'a str,
}

impl Workflow<'_> {
    fn evaluate(&self, p: &Product) -> &str {
        for rule in &self.rules {
            if let Some(state) = rule.evaluate(p) {
                return state;
            }
        }
        self.default
    }
}

#[derive(Default)]
struct Product {
    x: Value,
    m: Value,
    a: Value,
    s: Value,
}

impl Product {
    fn new(s: &str) -> Self {
        let mut product: Self = Default::default();
        for field in s[1..s.len() - 1].split(',') {
            let Some((key, value)) = field.split_once('=') else {
                unreachable!("must have equals")
            };
            let value = value.parse().unwrap();
            use Field::*;
            match Field::new(key.as_bytes()[0]) {
                X => product.x = value,
                M => product.m = value,
                A => product.a = value,
                S => product.s = value,
            }
        }
        product
    }

    fn get(&self, f: Field) -> Value {
        use Field::*;
        match f {
            X => self.x,
            M => self.m,
            A => self.a,
            S => self.s,
        }
    }

    fn value(&self) -> Value {
        self.x + self.m + self.a + self.s
    }
}

struct System<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    products: Vec<Product>,
}

impl<'a> System<'a> {
    fn new<'b>(s: &'b str) -> System<'b>
    where
        'b: 'a,
        'a: 'b,
    {
        let Some((workflows, products)) = s.split_once("\n\n") else {
            unreachable!("input")
        };
        let workflows = workflows
            .lines()
            .map(|l| {
                let Some((id, rules)) = l.split_once('{') else {
                    unreachable!("rule")
                };
                let mut rules = rules.trim_end_matches('}').split(',').collect_vec();
                let finally = rules.pop().unwrap();
                let rules = rules.iter().map(|r| Rule::new(r)).collect();
                (
                    id,
                    Workflow {
                        rules,
                        default: finally,
                    },
                )
            })
            .collect();

        let products = products.lines().map(Product::new).collect();

        Self {
            workflows,
            products,
        }
    }

    fn accept(&self, workflow: &str, p: &Product) -> bool {
        let mut workflow = workflow;
        loop {
            let flow = self.workflows.get(workflow).unwrap();
            let next = flow.evaluate(p);

            match next {
                "A" => return true,
                "R" => return false,
                _ => workflow = next,
            }
        }
    }

    fn accepted_sum(&self) -> Value {
        self.products
            .iter()
            .filter(|p| self.accept("in", p))
            .map(|p| p.value())
            .sum()
    }
}

#[derive(Clone, Copy)]
struct MinMaxValue {
    min: Value,
    max: Value,
}

impl Default for MinMaxValue {
    fn default() -> Self {
        Self { min: 1, max: 4000 }
    }
}

impl MinMaxValue {
    fn range(&self) -> Value {
        (self.max - self.min) + 1
    }
}

#[derive(Default, Clone, Copy)]
struct MinMaxProduct {
    x: MinMaxValue,
    m: MinMaxValue,
    a: MinMaxValue,
    s: MinMaxValue,
}

impl MinMaxProduct {
    fn get_mut(&mut self, f: Field) -> &mut MinMaxValue {
        use Field::*;
        match f {
            X => &mut self.x,
            M => &mut self.m,
            A => &mut self.a,
            S => &mut self.s,
        }
    }

    fn possible_combinations(&self) -> Value {
        self.x.range() * self.m.range() * self.a.range() * self.s.range()
    }
}

pub fn part_one(input: &str) -> Option<Value> {
    let system = System::new(input);
    Some(system.accepted_sum())
}

impl System<'_> {
    fn accept_minmax(&self, workflow: &str, mut range: MinMaxProduct) -> Value {
        let mut result = 0;
        let workflow = self.workflows.get(workflow).unwrap();

        for rule in &workflow.rules {
            let mut branch_range = range;
            let value = range.get_mut(rule.field);
            let branch_value = branch_range.get_mut(rule.field);
            match rule.op {
                Op::Gt => {
                    value.max = rule.value;
                    branch_value.min = rule.value + 1;
                }
                Op::Lt => {
                    value.min = rule.value;
                    branch_value.max = rule.value - 1;
                }
            }

            result += match rule.target {
                "A" => branch_range.possible_combinations(),
                "R" => 0,
                _ => self.accept_minmax(rule.target, branch_range),
            }
        }

        result += match workflow.default {
            "A" => range.possible_combinations(),
            "R" => 0,
            _ => self.accept_minmax(workflow.default, range),
        };

        result
    }

    fn possible_combinations(&self) -> Value {
        let product = MinMaxProduct::default();
        self.accept_minmax("in", product)
    }
}

pub fn part_two(input: &str) -> Option<Value> {
    let system = System::new(input);
    Some(system.possible_combinations())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167_409_079_868_000));
    }
}
