use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::cmp::Ordering;

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<Moon> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^<x=(?P<x>-?[0-9]*).*y=(?P<y>-?[0-9]*).*z=(?P<z>-?[0-9]*)>").unwrap();
    }

    let mut count = 0;

    input
        .lines()
        .filter(|l| RE.is_match(l))
        .map(|l| {
            let caps = RE.captures(l).unwrap();
            count = count + 1;
            Moon {
                pos_x: caps.name("x").unwrap().as_str().parse().unwrap(),
                pos_y: caps.name("y").unwrap().as_str().parse().unwrap(),
                pos_z: caps.name("z").unwrap().as_str().parse().unwrap(),
                vel_x: 0,
                vel_y: 0,
                vel_z: 0,
            }
        })
        .collect()
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Moon {
    pos_x: i32,
    pos_y: i32,
    pos_z: i32,
    vel_x: i32,
    vel_y: i32,
    vel_z: i32,
}
impl Moon {
    fn energy(&self) -> i32 {
        let pot = self.pos_x.abs() + self.pos_y.abs() + self.pos_z.abs();
        let kin = self.vel_x.abs() + self.vel_y.abs() + self.vel_z.abs();
        pot * kin
    }

    fn apply_gravity(&mut self, other: Moon) {
        self.vel_x += Self::compare_axis(self.pos_x, other.pos_x);
        self.vel_y += Self::compare_axis(self.pos_y, other.pos_y);
        self.vel_z += Self::compare_axis(self.pos_z, other.pos_z);
    }

    fn apply_gravity_single_axis(&mut self, other: Moon, axis: &char) {
        match axis {
            'x' => self.vel_x += Self::compare_axis(self.pos_x, other.pos_x),
            'y' => self.vel_y += Self::compare_axis(self.pos_y, other.pos_y),
            'z' => self.vel_z += Self::compare_axis(self.pos_z, other.pos_z),
            _ => (),
        }
    }
    fn apply_velocity_single_axis(&mut self, axis: &char) {
        match axis {
            'x' => self.pos_x += self.vel_x,
            'y' => self.pos_y += self.vel_y,
            'z' => self.pos_z += self.vel_z,
            _ => (),
        }
    }
    fn pos_value(&self, axis: &char) -> i32 {
        match axis {
            'x' => self.pos_x,
            'y' => self.pos_y,
            'z' => self.pos_z,
            _ => 0,
        }
    }
    fn vel_value(&self, axis: &char) -> i32 {
        match axis {
            'x' => self.vel_x,
            'y' => self.vel_y,
            'z' => self.vel_z,
            _ => 0,
        }
    }

    fn compare_axis(a: i32, b: i32) -> i32 {
        match a.cmp(&b) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }

    fn apply_velocity(&mut self) {
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;
        self.pos_z += self.vel_z;
    }
}

#[aoc(day12, part1)]
fn part1(moons: &[Moon]) -> i32 {
    run_x_times(1000, moons)
}

#[aoc(day12, part2)]
fn part2(moons: &[Moon]) -> u64 {
    run_until_match(moons)
}

fn run_x_times(times: usize, moons: &[Moon]) -> i32 {
    let mut moons: Vec<_> = moons.iter().map(|m| m.clone()).collect();

    for _ in 1..=times {
        //Gravity
        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                let m1 = moons[i];
                let m2 = moons[j];
                moons[i].apply_gravity(m2);
                moons[j].apply_gravity(m1);
            }
        }

        for moon in moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    moons.iter().map(|moon| moon.energy()).sum()
}


//Each of the dimensions requires a specific nuber of iterations to match 
//Once we know how many we need for each axis we just have to figure out the 
// lowest common multiple 
fn run_until_match(moons: &[Moon]) -> u64 {
    let mut results = vec![];

    for axis in ['x', 'y', 'z'].iter() {
        let mut moons = moons.to_vec();
        let match_axis = dimension_state(&moons, axis);

        let mut x_count = 0;

        loop {
            x_count += 1;
            for i in 0..moons.len() {
                for j in i + 1..moons.len() {
                    let m1 = moons[i];
                    let m2 = moons[j];
                    moons[i].apply_gravity_single_axis(m2, axis);
                    moons[j].apply_gravity_single_axis(m1, axis);
                }
            }

            for moon in moons.iter_mut() {
                moon.apply_velocity_single_axis(axis);
            }

            if match_axis == dimension_state(&moons, axis) {
                break;
            }
        }
        results.push(x_count);
        println!("axis:{:?} count:{:?}", axis, x_count);
    }

    let first = lcm(results[0], results[1]);
    lcm(first, results[2])
}

fn dimension_state(moons: &[Moon], axis: &char) -> [i32; 8] {
    [
        moons[0].pos_value(axis),
        moons[0].vel_value(axis),
        moons[1].pos_value(axis),
        moons[0].vel_value(axis),
        moons[2].pos_value(axis),
        moons[0].vel_value(axis),
        moons[3].pos_value(axis),
        moons[0].vel_value(axis),
    ]
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let data = parse_input(&SAMPLE_DATA);
        let res = part2(&data);

        assert_eq!(res, 2772);
    }

    #[test]
    fn part2_test_2() {
        let data = parse_input(&SAMPLE_2);
        let res = part2(&data);

        assert_eq!(res, 4686774924);
    }
    #[test]
    fn part1_test() {
        let data = parse_input(&SAMPLE_DATA);
        let res = run_x_times(10, &data);

        assert_eq!(res, 179);
    }
    #[test]
    fn test_parse() {
        let moons = parse_input(&SAMPLE_DATA);

        let h = vec![
            Moon {
                pos_x: -1,
                pos_y: 0,
                pos_z: 2,
                vel_x: 0,
                vel_y: 0,
                vel_z: 0,
            },
            Moon {
                pos_x: 2,
                pos_y: -10,
                pos_z: -7,
                vel_x: 0,
                vel_y: 0,
                vel_z: 0,
            },
            Moon {
                pos_x: 4,
                pos_y: -8,
                pos_z: 8,
                vel_x: 0,
                vel_y: 0,
                vel_z: 0,
            },
            Moon {
                pos_x: 3,
                pos_y: 5,
                pos_z: -1,
                vel_x: 0,
                vel_y: 0,
                vel_z: 0,
            },
        ];
        assert_eq!(moons, h);
    }

    lazy_static! {
        static ref SAMPLE_DATA: String = [
            "<x=-1, y=0, z=2>",
            "<x=2, y=-10, z=-7>",
            "<x=4, y=-8, z=8>",
            "<x=3, y=5, z=-1>",
        ]
        .join("\n");
        static ref SAMPLE_2: String = [
            "<x=-8, y=-10, z=0>",
            "<x=5, y=5, z=10>",
            "<x=2, y=-7, z=3>",
            "<x=9, y=-8, z=-3>",
        ]
        .join("\n");
    }
}
