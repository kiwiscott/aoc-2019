use crate::machine::Machine;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}
#[aoc(day7, part1)]
fn part1(instructions: &[i32]) -> i32 {
    let it : Vec<Vec<_>> = (0..5).permutations(5).collect(); 

    println!("Permutations: {:?}", it.len());

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let data = parse_input(&SAMPLE_DATA);
        assert_eq!(1, part1(&data));
    }

    lazy_static! {
        static ref SAMPLE_DATA: String = String::from("1002,4,3,4,33");
    }
}
