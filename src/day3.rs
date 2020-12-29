use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| {
                    let d = s.chars().take(1).next().unwrap();
                    let v = s.chars().skip(1).collect::<String>();

                    match d {
                        'L' => Direction::Left(v.parse().unwrap()),
                        'R' => Direction::Right(v.parse().unwrap()),
                        'U' => Direction::Up(v.parse().unwrap()),
                        'D' => Direction::Down(v.parse().unwrap()),
                        _ => unreachable!("Value not accounted for"),
                    }
                })
                .collect::<Vec<Direction>>()
        })
        .collect()
}
#[derive(Debug, Copy, Clone)]
enum Direction {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

#[aoc(day3, part1)]
fn part1(wires: &Vec<Vec<Direction>>) -> i32 {
    let paths = wire_points(wires);
    let p1: HashSet<(i32, i32)> = paths[0].iter().map(|p| *p).collect();
    let p2: HashSet<(i32, i32)> = paths[1].iter().map(|p| *p).collect();

    match p1
        .intersection(&p2)
        .into_iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
    {
        Some(n) => n,
        _ => panic!("Something bad happened"),
    }
}

#[aoc(day3, part2)]
fn part2(wires: &Vec<Vec<Direction>>) -> i32 {
    let paths = wire_points(wires);
    let p1: HashSet<(i32, i32)> = paths[0].iter().map(|p| *p).collect();
    let p2: HashSet<(i32, i32)> = paths[1].iter().map(|p| *p).collect();

    match p1
        .intersection(&p2)
        .into_iter()
        .map(|(x, y)| {
            let w1 = paths[0].iter().enumerate().find_map(|(index, (x1, y1))| {
                if x1 == x && y1 == y {
                    return Some(index + 1);
                } else {
                    None
                }
            });
            let w2 = paths[1].iter().enumerate().find_map(|(index, (x1, y1))| {
                if x1 == x && y1 == y {
                    return Some(index + 1);
                } else {
                    None
                }
            });
            w1.unwrap() + w2.unwrap()
        })
        .min()
    {
        Some(n) => n as i32,
        _ => panic!("Something bad happened"),
    }
}

fn wire_points(routes: &Vec<Vec<Direction>>) -> Vec<Vec<(i32, i32)>> {
    let paths: Vec<Vec<(i32, i32)>> = routes
        .iter()
        .map(|directions| {
            let mut points = vec![];
            let mut point = (0, 0);
            for d in directions.iter() {
                match d {
                    Direction::Up(n) => (0..*n).for_each(|_| {
                        point = (point.0, point.1 - 1);
                        points.push(point);
                    }),
                    Direction::Down(n) => (0..*n).for_each(|_| {
                        point = (point.0, point.1 + 1);
                        points.push(point);
                    }),
                    Direction::Left(n) => (0..*n).for_each(|_| {
                        point = (point.0 - 1, point.1);
                        points.push(point);
                    }),
                    Direction::Right(n) => (0..*n).for_each(|_| {
                        point = (point.0 + 1, point.1);
                        points.push(point);
                    }),
                };
            }
            points
        })
        .collect();
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = parse_input(&SAMPLE_DATA);
        assert_eq!(159, part1(&data));

        let data = parse_input(&SAMPLE_2);
        assert_eq!(135, part1(&data));
    }

    #[test]
    fn test_part2() {
        let data = parse_input(&SAMPLE_DATA);
        assert_eq!(610, part2(&data));

        let data = parse_input(&SAMPLE_2);
        assert_eq!(410, part2(&data));
    }

    lazy_static! {
        static ref SAMPLE_DATA: String = [
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83"
        ]
        .join("\n");
        static ref SAMPLE_2: String = [
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        ]
        .join("\n");
    }
}
