use crate::machine::{MachineState, VM};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::sync::mpsc::{Receiver, Sender};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(|c| c == ',' || c == '\r')
        .filter(|c| c != &"")
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn part1(instructions: &[i64]) -> i64 {
    //298586
    let it: Vec<Vec<_>> = (0..5).permutations(5).collect();
    let mut mr = MachineRunner::new(instructions);

    it.iter().map(|i| mr.run(i.to_vec())).max().unwrap_or(0)
}

#[aoc(day7, part2)]
fn part2(instructions: &[i64]) -> i64 {
    //9246095

    let it: Vec<Vec<_>> = (5..10).permutations(5).collect();
    let mut mr = MachineRunner::new(instructions);

    it.iter()
        .map(|i| mr.run_in_series(i.to_vec()))
        .max()
        .unwrap_or(0)
}

struct MachineRunner {
    instructions: Vec<i64>,
}
impl MachineRunner {
    fn new(instructions: &[i64]) -> Self {
        MachineRunner {
            instructions: instructions.to_vec(),
        }
    }

    fn run(&mut self, phase_setting: Vec<i64>) -> i64 {
        let mut previous_output: i64 = 0;
        for amplifier in 0..5 {
            let (mut vm, input, output) = VM::basic(&self.instructions);
            input
                .send(phase_setting[amplifier])
                .expect("Couldn't send inputs");
            input.send(previous_output).expect("Couldn't send inputs");
            //Run the machine
            vm.process();
            //Get the Results
            previous_output = output
                .try_recv()
                .expect("Amplifier did not retuan a result ");
        }
        previous_output
    }
    fn run_in_series(&mut self, phase_setting: Vec<i64>) -> i64 {
        let (mut amp1, amp1_input, amp1_ouput) = VM::basic(&self.instructions);

        amp1_input.send(phase_setting[0]).expect("Ok");
        amp1_input.send(0).expect("Ok");

        let (mut amp2, amp2_input, amp2_ouput) = VM::basic(&self.instructions);
        amp2_input.send(phase_setting[1]).expect("Ok");

        let (mut amp3, amp3_input, amp3_ouput) = VM::basic(&self.instructions);
        amp3_input.send(phase_setting[2]).expect("Ok");

        let (mut amp4, amp4_input, amp4_ouput) = VM::basic(&self.instructions);
        amp4_input.send(phase_setting[3]).expect("Ok");

        let (mut amp5, amp5_input, amp5_ouput) = VM::basic(&self.instructions);
        amp5_input.send(phase_setting[4]).expect("Ok");

        fn run_till_output(m1: &mut VM, m1_output: &Receiver<i64>, next_input: &Sender<i64>) {
            while m1.state() != MachineState::Halted {
                m1.tick();
                if let Ok(n) = m1_output.try_recv() {
                    next_input.send(n).expect("Sender not working");
                    return;
                }
            }
        }

        while amp5.state() != MachineState::Halted {
            run_till_output(&mut amp1, &amp1_ouput, &amp2_input);
            run_till_output(&mut amp2, &amp2_ouput, &amp3_input);
            run_till_output(&mut amp3, &amp3_ouput, &amp4_input);
            run_till_output(&mut amp4, &amp4_ouput, &amp5_input);
            run_till_output(&mut amp5, &amp5_ouput, &amp1_input);
        }

        amp5.last_output()
            .expect("Output expected from Amplifier 5")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part2_sample_1() {
        let sam = String::from(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        let data = parse_input(&sam);
        let mut mr = MachineRunner::new(&data);
        let x = mr.run_in_series(vec![9, 8, 7, 6, 5]);
        assert_eq!(139629729, x);
    }

    #[test]
    fn test_sample_1() {
        let sam = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let data = parse_input(&sam);
        let mut mr = MachineRunner::new(&data);
        let x = mr.run(vec![4, 3, 2, 1, 0]);
        assert_eq!(43210, x);
    }

    #[test]
    fn test_sample_3() {
        let sam = String::from(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        let data = parse_input(&sam);
        let mut mr = MachineRunner::new(&data);
        let x = mr.run(vec![0, 1, 2, 3, 4]);
        assert_eq!(54321, x);
    }

    #[test]
    fn test_sample_2() {
        let sam = String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        let data = parse_input(&sam);
        let mut mr = MachineRunner::new(&data);
        let x = mr.run(vec![1, 0, 4, 3, 2]);
        assert_eq!(65210, x);
    }
}
