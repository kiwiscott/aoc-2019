use crate::machine::{MachineState, VM};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(|c| c == ',' || c == '\r')
        .filter(|c| c != &"")
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

type Point = (i32, i32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color {
    White,
    Black,
}
impl Color {
    fn from(i: i64) -> Self {
        if i == 0 {
            return Color::Black;
        }
        Color::White
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn from(i: i64) -> Self {
        if i == 0 {
            return Direction::Left;
        }
        Direction::Right
    }

    fn move_forward(&self, point: Point) -> Point {
        match self {
            Direction::Left => (point.0 - 1, point.1),
            Direction::Right => (point.0 + 1, point.1),
            Direction::Up => (point.0, point.1 - 1),
            Direction::Down => (point.0, point.1 + 1),
        }
    }
    fn turn(&self, direction: Direction) -> Direction {
        match (self, direction) {
            (Direction::Left, Direction::Left) => Direction::Down,
            (Direction::Left, Direction::Right) => Direction::Up,
            (Direction::Right, Direction::Left) => Direction::Up,
            (Direction::Right, Direction::Right) => Direction::Down,
            (Direction::Down, Direction::Left) => Direction::Right,
            (Direction::Down, Direction::Right) => Direction::Left,
            (Direction::Up, Direction::Left) => Direction::Left,
            (Direction::Up, Direction::Right) => Direction::Right,
            _ => Direction::Up,
        }
    }
}

#[aoc(day11, part1)]
fn part1(instructions: &[i64]) -> usize {
    let ship_map = run_the_robot(instructions, 0);
    ship_map.len()
}
#[aoc(day11, part2)]
fn part2(instructions: &[i64]) -> usize {
    let ship_map = run_the_robot(instructions, 1);

    let mut min_x = 1;
    let mut min_y = 1;
    let mut max_x = -1;
    let mut max_y = -1;

    for (x, y) in ship_map.keys() {
        if x < &min_x {
            min_x = *x
        };
        if x > &max_x {
            max_x = *x
        };
        if y < &min_y {
            min_y = *y
        };
        if y > &max_y {
            max_y = *y
        };
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = (x, y);
            let c = ship_map.get(&p).unwrap_or(&Color::Black);
            match c {
                Color::Black => print!(" "),
                Color::White => print!("#"),
            }
        }
        print!("\n")
    }
    ship_map.len()
}
fn run_the_robot(instructions: &[i64], first_input: i64) -> HashMap<Point, Color> {
    let (mut vm, input, output) = VM::basic(instructions);

    input.send(first_input).unwrap();

    let mut first = true;
    let mut position = (0, 0);
    let mut facing = Direction::Up;
    let mut ship_map: HashMap<Point, Color> = HashMap::new();

    while vm.state() != MachineState::Halted {
        vm.tick();
        if let Ok(n) = output.try_recv() {
            if first {
                first = false;
                ship_map.insert(position, Color::from(n));
            } else {
                let direction_to_turn = Direction::from(n);
                facing = facing.turn(direction_to_turn);
                position = facing.move_forward(position);
                let on_color = ship_map.get(&position).unwrap_or(&Color::Black);

                if on_color == &Color::Black {
                    input.send(0).unwrap();
                } else {
                    input.send(1).unwrap();
                }

                first = true;
            }
        }
    }
    ship_map
}
