use crate::machine::{Machine, MachineState};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(|c| c == ',' || c == '\r')
        .filter(|c| c != &"")
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}
#[aoc(day9, part1)]
fn part1(instructions: &[i64]) -> i64 {
    let inputs = vec![1];
    let mut m = Machine::new(instructions.to_vec(), inputs);
    m.process();
    for o in m.outputs() {
        println!("--{:?}", o);
    }
    if m.state != MachineState::Halted {
        panic!("NOT DONE");
    }
    *m.outputs().last().unwrap_or(&0)
}

#[aoc(day9, part2)]
fn part2(instructions: &[i64]) -> i64 {
    let inputs = vec![2];
    let mut m = Machine::new(instructions.to_vec(), inputs);
    m.process();
    for o in m.outputs() {
        println!("--{:?}", o);
    }
    if m.state != MachineState::Halted {
        panic!("NOT DONE");
    }
    *m.outputs().last().unwrap_or(&0)
}
