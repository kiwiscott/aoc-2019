use crate::machine::Machine;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}
#[aoc(day5, part1)]
fn part1(instructions: &[i32]) -> i32 {
    fn next() -> &'static str {
        "1"
    }
    fn write(s: &str) {
        println!("Diagnostic Code: {}", s);
    }

    let mut m = Machine::new(&instructions);
    m.input = Box::new(next);
    m.output = Box::new(write);
    m.process();
    0
}

#[aoc(day5, part2)]
fn part2(instructions: &[i32]) -> i32 {
    fn next() -> &'static str {
        "5"
    }
    fn write(s: &str) {
        println!("Diagnostic Code: {}", s);
    }

    let mut m = Machine::new(&instructions);
    m.input = Box::new(next);
    m.output = Box::new(write);
    m.process();
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let data = parse_input(&SAMPLE_DATA);

        fn next() -> &'static str {
            "1"
        }
        let mut m = Machine::new(&data);
        m.input = Box::new(next);
        m.process();
        assert_eq!(99, m.value_at(4));
    }

    lazy_static! {
        static ref SAMPLE_DATA: String = String::from("1002,4,3,4,33");
    }
}
