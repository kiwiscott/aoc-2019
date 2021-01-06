use crate::machine::{MachineState, VM};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{Receiver, Sender};

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(|c| c == ',' || c == '\r')
        .filter(|c| c != &"")
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Found {
    Open,
    Wall,
    Oxygen,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn to(&self) -> i64 {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }
    fn next_point(&self, origin: Point) -> Point {
        match self {
            Self::North => Point::new(origin.x, origin.y - 1),
            Self::South => Point::new(origin.x, origin.y + 1),
            Self::West => Point::new(origin.x - 1, origin.y),
            Self::East => Point::new(origin.x + 1, origin.y),
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

type Map = HashMap<Point, Found>;
trait Neighbours {
    fn neighbours(&self, point: Point) -> Vec<Point>;
}
impl Neighbours for Map {
    fn neighbours(&self, point: Point) -> Vec<Point> {
        let mut n = vec![];
        for dir in &[
            Direction::East,
            Direction::North,
            Direction::West,
            Direction::South,
        ] {
            let np = dir.next_point(point);
            match self.get(&np) {
                Some(Found::Open) => n.push(np),
                Some(Found::Oxygen) => n.push(np),
                _ => (),
            }
        }

        n
    }
}

struct Mapper {
    droid: VM,
    input: Sender<i64>,
    output: Receiver<i64>,
}
impl Mapper {
    fn new(instructions: &[i64]) -> Mapper {
        let (vm, input, output) = VM::basic(&instructions);
        Mapper {
            droid: vm,
            input: input,
            output: output,
        }
    }

    fn discover_map(&mut self) -> Map {
        let mut map = Map::new();
        let origin = Point::new(0, 0);
        map.insert(origin, Found::Open);
        self.visit(&mut map, origin);

        map
    }

    fn visit(&mut self, map: &mut Map, pos: Point) {
        for dir in &[
            Direction::East,
            Direction::North,
            Direction::West,
            Direction::South,
        ] {
            let new_pos = dir.next_point(pos);

            if !map.contains_key(&new_pos) {
                let status = self.step(dir.to());

                map.insert(new_pos, status);

                if status != Found::Wall {
                    self.visit(map, new_pos);
                    self.step(dir.opposite().to());
                }
            }
        }
    }

    fn step(&mut self, movement: i64) -> Found {
        self.input.send(movement).unwrap();
        while self.droid.state() != MachineState::Halted {
            self.droid.tick();
            if let Ok(n) = self.output.try_recv() {
                return match n {
                    0 => Found::Wall,
                    1 => Found::Open,
                    2 => Found::Oxygen,
                    _ => unimplemented!(),
                };
            }
        }
        Found::Oxygen
    }
}

fn finder(came_from: HashSet<Point>, src: Point, dest: Point, map: &Map) -> Option<i32> {
    if src == dest {
        return Some(came_from.len() as i32);
    }

    map.neighbours(src)
        .iter()
        .filter_map(|neigh| {
            let mut came_from = came_from.clone();
            if came_from.insert(*neigh) {
                return finder(came_from, *neigh, dest, &map);
            }
            None
        })
        .min()
}
/* 
fn print_map(map: &Map) {
    println!("Map\n",);
    for i in 0..40 {
        let mut s = String::new();
        for j in 0..40 {
            let p = Point::new(j - 20, i - 20);
            if p.x == 0 && p.y == 0 {
                s.push('@');
                continue;
            }

            match &map.get(&p) {
                Some(Found::Wall) => s.push('#'),
                Some(Found::Open) => s.push(' '),
                Some(Found::Oxygen) => s.push('X'),
                _ => s.push('~'),
            }
        }
        println!("{}", s);
    }
} */

#[aoc(day15, part1)]
fn part1(instructions: &[i64]) -> i32 {
    let mut mapper = Mapper::new(instructions);
    let map = mapper.discover_map();

    //Find Path
    let origin = Point::new(0, 0);
    let dest = map
        .iter()
        .find(|(_point, position)| **position == Found::Oxygen)
        .expect("Trouble");

    //Shortest Path
    let mut came_from = HashSet::new();
    came_from.insert(origin);
    //We inserted so remove one at the end
    let path_len = -1 + finder(came_from, origin, *dest.0, &map).expect("Should have found a path");

    path_len
}

#[aoc(day15, part2)]
fn part2(instructions: &[i64]) -> i32 {
    let mut mapper = Mapper::new(instructions);
    let mut map = mapper.discover_map();
    
    let dest = &map
        .iter()
        .find(|(_point, position)| position == &&Found::Oxygen)
        .expect("Trouble");

    let mut just_filled = vec![*dest.0];
    let mut cycles = -1;

    while !just_filled.is_empty() {
        cycles += 1;

        for _ in 0..just_filled.len() {
            let current = just_filled.remove(0);
            for dir in &[
                Direction::East,
                Direction::North,
                Direction::West,
                Direction::South,
            ] {
                let next = dir.next_point(current);

                loop {
                    if let Some(x) = map.get_mut(&next) {
                        if *x == Found::Open {
                            *x = Found::Oxygen;
                            just_filled.push(next.clone());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        //print_map(&map); 
    }
    cycles
}

/*
he remote control program executes the following steps in a loop forever:

Accept a movement command via an input instruction.
Send the movement command to the repair droid.
Wait for the repair droid to finish the movement operation.
Report on the status of the repair droid via an output instruction.
Only four movement commands are understood: north (1), south (2), west (3), and east (4). Any other command is invalid. The movements differ in direction,
but not in distance: in a long enough east-west hallway, a series of commands like 4,4,4,4,3,3,3,3 would leave the repair droid back where it started.

The repair droid can reply with any of the following status codes:

0: The repair droid hit a wall. Its position has not changed.
1: The repair droid has moved one step in the requested direction.
2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
You don't know anything about the area around the repair droid, but you can figure it out by watching the status codes.

For example, we can draw the area using D for the droid, # for walls, . for locations the droid can traverse, and empty space for unexplored locations. Then, the initial state looks like this:
*/
