use crate::machine::VM;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(|c| c == ',' || c == '\r')
        .filter(|c| c != &"")
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}
#[aoc(day5, part1)]
fn part1(instructions: &[i64]) -> i64 {
    run(instructions, &[1])
}

#[aoc(day5, part2)]
fn part2(instructions: &[i64]) -> i64 {
    run(instructions, &[5])
}

fn run(instructions: &[i64], inputs: &[i64]) -> i64 {
    let (mut vm, input,_o) = VM::basic(instructions); 
    for i in inputs{
        input.send(*i).expect("HUH?");
    }        
    vm.process();
    vm.last_output().expect("Output expected")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let data = parse_input(&SAMPLE_DATA);
        let x = run(&data, &[8]);
        assert_eq!(1, x);
    }

    lazy_static! {
        static ref SAMPLE_DATA: String = String::from("3,9,8,9,10,9,4,9,99,-1,8");
    }
}
