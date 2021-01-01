use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use std::collections::HashSet;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Space {
    input
        .lines()
        .enumerate()
        .map(|(yi, row)| {
            row.chars()
                .enumerate()
                .map(|(xi, c)| Segment::new(xi as u16, yi as u16, c == '#'))
                .collect::<Space>()
        })
        .flatten()
        .collect()
}

type Space = Vec<Segment>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Segment {
    x: u16,
    y: u16,
    contains_asteriod: bool,
}
impl Segment {
    fn new(x: u16, y: u16, contains_asteriod: bool) -> Segment {
        Segment {
            x: x,
            y: y,
            contains_asteriod: contains_asteriod,
        }
    }
    fn has_same_coordinates(&self, other: &Segment) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn manhattan_from(&self, other: &Segment) -> i32 {
        (self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()
    }
}

#[aoc(day10, part1)]
fn part1(asteroid_field: &Space) -> usize {
    let l = asteroid_field
        .iter()
        .filter(|segment| segment.contains_asteriod)
        .map(|origin| {
            let h = asteroid_field
                .iter()
                .filter(|destination| {
                    destination.contains_asteriod && !destination.has_same_coordinates(origin)
                })
                .fold(HashSet::new(), |mut acc, destination| {
                    let d = heading_degress(origin, destination);
                    acc.insert(d.to_string());
                    acc
                });
            (origin, h.len())
        })
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .last();

    match l {
        Some((_segment, count)) => count,
        None => 0,
    }
}

#[aoc(day10, part2)]
fn part2(asteroid_field: &Space) -> u16 {
    let origin = Segment::new(8, 16, true);

    match destory_find_nth(origin, asteroid_field, 200) {
        Some(seg) => (seg.x * 100) + seg.y,
        None => panic!("No Result Found"),
    }
}

fn destory_find_nth(origin: Segment, asteroid_field: &Space, find_nth: u32) -> Option<&Segment> {
    let mut segments: Vec<(&Segment, f32, i32)> = asteroid_field
        .iter()
        .filter(|segment| segment.contains_asteriod)
        .map(|dest| {
            (
                dest,
                heading_degress(&origin, dest),
                dest.manhattan_from(&origin),
            )
        })
        //if we order by manhattan distance we are guaranteed to come across the closest one first.
        .sorted_by(|a, b| a.2.cmp(&b.2))
        .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .collect();

    let mut nth = None;
    let mut count = 0;
    let mut filtered = HashSet::new();

    let mut i = 0;
    while count < find_nth {
        while i != segments.len() {
            let seg = &mut segments[i];

            if filtered.insert(seg.1.to_string()) {
                //this means is wasn't there
                count += 1;
                if count == find_nth {
                    nth = Some(seg.0);
                    break;
                }
                segments.remove(i);
            } else {
                i += 1;
            }
        }
        filtered.clear();
    }

    nth
}

fn heading_degress(origin: &Segment, destination: &Segment) -> f32 {
    let delta_x = origin.x as f32 - destination.x as f32;
    let delta_y = origin.y as f32 - destination.y as f32;

    let rad = delta_y.atan2(delta_x);
    let d = (rad.to_degrees() + 360.0) % 360.0;

    if d < 90.0 {
        return 270.0 + d;
    }
    d - 90.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let i = parse_input(&SAMPLE);
        let xy = part1(&i);

        assert_eq!(8, xy);
    }
    #[test]
    fn test_part2() {
        let data = parse_input(&SAMPLE2);
        let origin = Segment::new(11, 13, true);
        let nth = destory_find_nth(origin, &data, 200);

        let nth = nth.unwrap();

        assert_eq!(8, nth.x);
        assert_eq!(2, nth.y);
    }
    #[test]

    fn test_coords() {
        //let i = parse_input(&SIMPLE);
        let origin = Segment::new(2, 2, true);

        let north = Segment::new(2, 1, true);
        assert_eq!(0.0, heading_degress(&origin, &north));

        let south = Segment::new(2, 3, true);
        assert_eq!(180.0, heading_degress(&origin, &south));

        let east = Segment::new(3, 2, true);
        assert_eq!(90.0, heading_degress(&origin, &east));

        let west = Segment::new(1, 2, true);
        assert_eq!(270.0, heading_degress(&origin, &west));

        let northeast = Segment::new(3, 1, true);
        assert_eq!(45.0, heading_degress(&origin, &northeast));

        let southeast = Segment::new(3, 3, true);
        assert_eq!(135.0, heading_degress(&origin, &southeast));

        let southwest = Segment::new(1, 3, true);
        assert_eq!(225.0, heading_degress(&origin, &southwest));

        let south_southwest = Segment::new(1, 4, true);
        assert_eq!(206.56506, heading_degress(&origin, &south_southwest));

        let northwest = Segment::new(1, 1, true);
        assert_eq!(315.0, heading_degress(&origin, &northwest));

        let north_northwest = Segment::new(1, 0, true);
        assert_eq!(333.43494, heading_degress(&origin, &north_northwest));

        let west_northwest = Segment::new(0, 1, true);
        assert_eq!(296.56506, heading_degress(&origin, &west_northwest));
    }

    lazy_static! {
        static ref SAMPLE: String = [".#..#", ".....", "#####", "....#", "...##",].join("\n");
        static ref SIMPLE: String = [
            ".#....#####...#..",
            "##...##.#####..##",
            "##...#...#.#####.",
            "..#.....X...###..",
            "..#.#.....#....##",
        ]
        .join("\n");
        static ref SAMPLE2: String = [
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ]
        .join("\n");
    }
}
