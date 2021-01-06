use std::sync::mpsc::Sender;
use crate::machine::{MachineState, VM};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(|c| c == ',' || c == '\r')
        .filter(|c| c != &"")
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}


#[aoc(day17, part2)]
fn part2(instructions: &[i64]) -> i64 {
    //let mut data = vec![];

    let mut ins = instructions.to_vec();
    ins[0] = 2;
    let (mut vm, input, _output) = VM::basic(&ins);

    //Printed out the steps and then used VSCODE to compress them and my head.
    //It wasn't until I compressed them in my brain that I could see the code I could use.
    send_string(&input, "A,B,B,C,C,A,B,B,C,A\n");
    send_string(&input, "R,4,R,12,R,10,L,12\n");
    send_string(&input, "L,12,R,4,R,12\n");
    send_string(&input, "L,12,L,8,R,10\n");
    send_string(&input, "n\n");

    //let mut collector = vec![];
    while vm.state() != MachineState::Halted {
        vm.tick();
        //Used alot during th figure out the steps!!  
        //if let Ok(n) = _output.try_recv() {
        //    print!("{}", n as u8 as char);
        //}
    }

    vm.last_output().unwrap()
}

pub fn send_string(input: &Sender<i64>, s: &str) {
    for c in s.chars() {
        let data = c as i64;
        input.send(data).expect("Should work");
    }
}



#[aoc(day17, part1)]
fn part1(instructions: &[i64]) -> usize {
    let mut data = vec![];
    let (mut vm, _input, output) = VM::basic(instructions);

    let mut collector = vec![];
    while vm.state() != MachineState::Halted {
        vm.tick();
        if let Ok(n) = output.try_recv() {
            if n == 10 {
                if collector.len() != 0 {
                    data.push(collector);
                }
                collector = vec![];
            } else {
                collector.push(n as u8 as char);
            }
        }
    }

    let offsets = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut points = vec![];

    for (y, s) in data.iter().enumerate() {
        for (x, c) in s.iter().enumerate() {
            if c == &'#' {
                let cross_road = offsets.iter().all(|(x_offset, y_offset)| {
                    if x == 0 || y == 0 {
                        return false;
                    }
                    let y = (y as i32 + y_offset) as usize;
                    let x = (x as i32 + x_offset) as usize;

                    match data.get(y) {
                        None => false,
                        Some(row) => match row.get(x) {
                            Some('#') => true,
                            _ => false,
                        },
                    }
                });
                if cross_road {
                    points.push((x, y));
                }
            };
        }
    }

    points.iter().map(|(x, y)| x * y).sum()
}
