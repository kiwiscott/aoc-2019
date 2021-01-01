use crate::machine::{MachineState, VM};
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
    let (mut vm, input, _o) = VM::basic(instructions);

    input.send(1).unwrap();
    vm.process();
    assert!(vm.state() == MachineState::Halted, "Machine is still running");
    vm.last_output().expect("Output expected")
}

#[aoc(day9, part2)]
fn part2(instructions: &[i64]) -> i64 {
    let (mut vm, input, output) = VM::basic(instructions);
    input.send(1).unwrap();
    vm.process();

    println!("More than one output is an error:");
    while let Ok(n) = output.try_recv() {
        println!("\t{:?}", n);
    }
    assert!(vm.state() == MachineState::Halted, "Machine is still running");
    vm.last_output().expect("Output expected")
}
