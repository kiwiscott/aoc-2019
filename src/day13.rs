use crate::machine::{MachineState, VM};
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(|c| c == ',' || c == '\r')
        .filter(|c| c != &"")
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day13, part1)]
fn part1(instructions: &[i64]) -> usize {
    let mut data = vec![];
    let (mut vm, _input, output) = VM::basic(instructions);
    while vm.state() != MachineState::Halted {
        vm.tick();
        if let Ok(n) = output.try_recv() {
            data.push(n);
        }
    }

    data.chunks(3).filter(|p| p[2] == 2).count()
}

#[aoc(day13, part2)]
fn part2(instructions: &[i64]) -> i64 {
    let mut instructions = instructions.to_vec();
    instructions[0] = 2;

    let mut data = vec![];
    let mut paddle = (0, 0);
    let (mut vm, input, output) = VM::basic(&instructions);
    while vm.state() != MachineState::Halted {
        vm.tick();
        if let Ok(n) = output.try_recv() {
            data.push(n);
            if data.len() != 3 {
                continue;
            }

            //PLay th game -- moving the paddle to the ball
            if data[2] == 3 {
                paddle = (data[0], data[1])
            } else if data[2] == 4 {
                let move_dir = match paddle.0.cmp(&data[0]) {
                    Ordering::Less => 1, //Right
                    Ordering::Equal => 0,
                    Ordering::Greater => -1, //Left
                };
                input.send(move_dir).unwrap();
            }
            //println!("{:?}", data);
            data.clear();
        }
    }
    vm.last_output().unwrap()
}
