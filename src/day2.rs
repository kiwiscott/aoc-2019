use crate::machine::Machine;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(instructions: &[i64]) -> i64 {
    let mut v = instructions.to_vec();
    v[1] = 12;
    v[2] = 2;
    let mut m = Machine::new(v, vec![]);
    m.process();
    m.value_at(0)
}
#[aoc(day2, part2)]

fn part2(instructions: &[i64]) -> i64 {
    let result = (0..100).cartesian_product(0..100).find_map(|(i, j)| {
        let mut v = instructions.to_vec();
        v[1] = i;
        v[2] = j;
        let mut m = Machine::new(v, vec![]);
        m.process();

        match 19690720 == m.value_at(0) {
            true => Some((i * 100) + j),
            false => None,
        }
    });

    match result {
        Some(n) => n,
        None => panic!("No result could be found!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = parse_input(&SAMPLE_DATA);

        let mut m = Machine::new(data, vec![]);
        m.process();
        assert_eq!(3500, m.value_at(0));
    }

    lazy_static! {
        static ref SAMPLE_DATA: String = String::from("1,9,10,3,2,3,11,0,99,30,40,50");
    }
}
