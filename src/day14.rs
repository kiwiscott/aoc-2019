use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day14)]
fn parse_input(input: &str) -> HashMap<String, Reaction> {
    let mut reactions = HashMap::new();
    for line in input.lines() {
        let mut v = line.split("=>");
        let left: Vec<&str> = v.next().unwrap().split(',').collect();
        let mut right = v.next().unwrap().split_ascii_whitespace();

        let produces = Element {
            qty: right.next().unwrap().parse().unwrap(),
            name: right.next().unwrap().to_string(),
        };

        let requires = left
            .iter()
            .map(|e| {
                let mut ie = e.split_ascii_whitespace();
                Element {
                    qty: ie.next().unwrap().parse().unwrap(),
                    name: ie.next().unwrap().to_string(),
                }
            })
            .collect();
        reactions.insert(
            produces.name.clone(),
            Reaction {
                produces: produces,
                requires: requires,
            },
        );
    }
    reactions
}

#[derive(Debug)]
struct Reaction {
    produces: Element,
    requires: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Element {
    qty: i64,
    name: String,
}

impl Element {
    fn multiple(&self, num: i64) -> Self {
        Element {
            qty: self.qty * num,
            name: self.name.clone(),
        }
    }
}

#[aoc(day14, part1)]
fn part1(reactions: &HashMap<String, Reaction>) -> i64 {
    let one_fuel = Element {
        qty: 1,
        name: "FUEL".to_string(),
    };

    required_ore(one_fuel, reactions)
}

#[aoc(day14, part2)]
fn part2(reactions: &HashMap<String, Reaction>) -> i64 {
    let max = 1_000_000_000_000_i64;
    let mut start = 0;
    let mut end = max;

    loop {
        let one_fuel = Element {
            qty: (end + start) / 2,
            name: "FUEL".to_string(),
        };
        let total = required_ore(one_fuel, reactions);

        if total > max {
            end = (end + start) / 2;
        } else {
            start = (end + start) / 2;
        }

        if end - start == 1 || end == start {
            return start;
        }
    }
}

fn ciel(a: &i64, b: &i64) -> i64 {
    let x = a / b;
    if x * b < *a {
        x + 1
    } else {
        x
    }
}

fn required_ore(element: Element, reactions: &HashMap<String, Reaction>) -> i64 {
    let mut still_required = vec![element];
    let mut ore = 0;
    let mut left_over_elements: HashMap<String, i64> = HashMap::new();

    while let Some(required) = still_required.pop() {
        if required.name == "ORE" {
            ore += required.qty;
            continue;
        }
        //Total Quantity we need to be able to create. Reuse from the cache if we can
        let mut qty_needed = required.qty;
        if let Some(left_over_qty) = left_over_elements.get_mut(&required.name) {
            if qty_needed <= *left_over_qty {
                *left_over_qty -= qty_needed;
                qty_needed = 0;
            } else {
                qty_needed -= *left_over_qty;
                left_over_elements.remove(&required.name);
            }
        }

        let reaction = reactions
            .get(&required.name)
            .expect("Reaction to produce ingredient does not exist.");

        //We only have to run this a number of times based on the total qty needed
        let batches_required_reaction = ciel(&qty_needed, &reaction.produces.qty);

        //We may have left overs from the process - save them for later
        let for_later = (reaction.produces.qty * batches_required_reaction) - qty_needed;
        if for_later > 0 {
            *left_over_elements.entry(required.name).or_insert(0) += for_later;
        }

        for requirement in reaction.requires.iter() {
            still_required.push(requirement.multiple(batches_required_reaction));
        }
    }
    return ore;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let elements = parse_input(&SAMPLE_DATA);
        assert_eq!(part1(&elements), 31);
    }

    #[test]
    fn test_part_2() {
        let elements = parse_input(&SAMPLE_DATA_2);
        assert_eq!(part1(&elements), 165);
    }

    #[test]
    fn test_parse() {
        let elements = parse_input(&SAMPLE_DATA);

        assert_eq!(elements.len(), 6);
    }

    lazy_static! {
        static ref SAMPLE_DATA_2: String = [
            "9 ORE => 2 A",
            "8 ORE => 3 B",
            "7 ORE => 5 C",
            "3 A, 4 B => 1 AB",
            "5 B, 7 C => 1 BC",
            "4 C, 1 A => 1 CA",
            "2 AB, 3 BC, 4 CA => 1 FUEL",
        ]
        .join("\n");
        static ref SAMPLE_DATA: String = [
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        ]
        .join("\n");
    }
}
