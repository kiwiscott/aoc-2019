use crate::machine::VM;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(|c| c == ',' || c == '\r')
        .filter(|c| c != &"")
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(instructions: &[i64]) -> i64 {
    vm_replace(instructions, 12, 2, true)
}

#[aoc(day2, part2)]

fn part2(instructions: &[i64]) -> i64 {
    let result = (0..100).cartesian_product(0..100).find_map(|(i, j)| {
        match 19690720 == vm_replace(instructions, i, j, true) {
            true => Some((i * 100) + j),
            false => None,
        }
    });

    match result {
        Some(n) => n,
        None => panic!("No result could be found!"),
    }
}

fn vm_replace(instructions: &[i64], replace1: i64, replace2: i64, do_replace: bool) -> i64 {
    let mut v = instructions.to_vec();
    if do_replace {
        v[1] = replace1;
        v[2] = replace2;
    }
    let (mut vm, _i, _o) = VM::basic(&v);
    vm.process();
    *vm.instruction_at(0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = parse_input(&SAMPLE_DATA);
        let o = vm_replace(&data, 0, 0, false);

        assert_eq!(3500, o);
    }
    #[test]
    fn test_replace() {
        let data = parse_input(&REPLACE_DATA);
        let o = vm_replace(&data, 9, 10, true);
        assert_eq!(4950, o);
    }

    lazy_static! {
        static ref SAMPLE_DATA: String = String::from("1,9,10,3,2,3,11,0,99,30,40,50");
        static ref REPLACE_DATA: String = String::from("1,0,0,3,2,3,11,0,99,45,45,55");

    }
}
