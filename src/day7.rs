use crate::machine::{Machine, MachineState};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(|c| c == ',' || c == '\r')
        .filter(|c| c != &"")
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn part1(instructions: &[i32]) -> i32 {
    let it: Vec<Vec<_>> = (0..5).permutations(5).collect();
    let mut mr = MachineRunner::new(instructions);

    it.iter().map(|i| mr.run(i.to_vec())).max().unwrap_or(0)
}

#[aoc(day7, part2)]
fn part2(instructions: &[i32]) -> i32 {
    let it: Vec<Vec<_>> = (5..10).permutations(5).collect();
    let mut mr = MachineRunner::new(instructions);

    it.iter()
        .map(|i| mr.run_in_series(i.to_vec()))
        .max()
        .unwrap_or(0)
}

struct MachineRunner {
    instructions: Vec<i32>,
    memo: HashMap<String, i32>,
}
impl MachineRunner {
    fn new(instructions: &[i32]) -> Self {
        MachineRunner {
            instructions: instructions.to_vec(),
            memo: HashMap::<String, i32>::new(),
        }
    }

    fn run(&mut self, phase_setting: Vec<i32>) -> i32 {
        let mut previous_output = 0;
        for amplifier in 0..5 {
            let inputs = vec![phase_setting[amplifier], previous_output];

            let key = format!("{:?}_{:?}", phase_setting[amplifier], previous_output);

            match self.memo.get(&key) {
                Some(n) => previous_output = *n,
                None => {
                    let mut m = Machine::new(self.instructions.clone(), inputs);
                    let mut m_output_ln = m.outputs().len();
                    while m.state != MachineState::Halted {
                        m.next();
                        //We ave to get the putput and put it into the input
                        let outs = m.outputs();
                        for i in m_output_ln..outs.len() {
                            m.insert_input(outs[i]);
                        }
                        m_output_ln = outs.len();
                    }
                    previous_output = *m.outputs().last().expect("No output from the machine");
                    self.memo.insert(key, previous_output);
                }
            }
        }
        previous_output
    }
    fn run_in_series(&mut self, phase_setting: Vec<i32>) -> i32 {
        let mut ma = Machine::new(self.instructions.clone(), vec![phase_setting[0], 0]);
        let mut mb = Machine::new(self.instructions.clone(), vec![phase_setting[1]]);
        let mut mc = Machine::new(self.instructions.clone(), vec![phase_setting[2]]);
        let mut md = Machine::new(self.instructions.clone(), vec![phase_setting[3]]);
        let mut me = Machine::new(self.instructions.clone(), vec![phase_setting[4]]);

        let mut ma_out_len = 0;
        let mut mb_out_len = 0;
        let mut mc_out_len = 0;
        let mut md_out_len = 0;
        let mut me_out_len = 0;

        fn run_till_output(m1: &mut Machine, m2: &mut Machine, m1_len: &mut usize) {
            while m1.state != MachineState::Halted {
                m1.next();
                let outs = m1.outputs();
                if outs.len() > *m1_len {
                    m2.insert_input(*outs.last().unwrap());
                    *m1_len = outs.len();
                    return;
                }
            }
        }

        while me.state != MachineState::Halted {
            run_till_output(&mut ma, &mut mb, &mut ma_out_len);
            run_till_output(&mut mb, &mut mc, &mut mb_out_len);
            run_till_output(&mut mc, &mut md, &mut mc_out_len);
            run_till_output(&mut md, &mut me, &mut md_out_len);
            run_till_output(&mut me, &mut ma, &mut me_out_len);
        }
        *me.outputs().last().unwrap_or(&0)
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
