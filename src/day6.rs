use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Orbits {
    let mut res = Orbits::new();

    for line in input.lines() {
        let v: Vec<&str> = line.splitn(2, ')').collect();
        let vertex = res
            .entry(v[0].to_string())
            .or_insert(HashSet::<String>::new());
        vertex.insert(v[1].to_string());
    }
    res
}

type Orbits = HashMap<String, HashSet<String>>;

#[aoc(day6, part1)]
fn part1(orbits: &Orbits) -> i32 {
    count_sub_orbits(&0, "COM", orbits)
}

fn count_sub_orbits(level: &i32, orbit: &str, orbits: &Orbits) -> i32 {
    let mut c = *level;
    if orbits.contains_key(orbit) {
        c += orbits.get(orbit).unwrap().iter().fold(0, |mut acc, sub| {
            let level = level + 1;
            acc += count_sub_orbits(&level, sub, orbits);
            acc
        });
    };
    c
}
#[aoc(day6, part2)]
fn part2(orbits: &Orbits) -> i32 {
    let p = PathFinder::new(orbits);
    p.find_shortest_path("YOU", "SAN")
}

struct PathFinder {
    orbits: Orbits,
}
impl PathFinder {
    fn new(orbits: &Orbits) -> Self {
        Self {
            orbits: orbits.clone(),
        }
    }
    fn find_shortest_path(&self, from: &str, destination: &str) -> i32 {
        let mut visited = HashSet::<String>::new();
        let path_len = self.path(from, destination, &mut visited);
        //we only want to swap the parents to be the same so lets remove the cild nores count
        path_len - 2
    }
    fn path(&self, from: &str, destination: &str, visited: &mut HashSet<String>) -> i32 {
        if from == destination {
            return visited.len() as i32;
        }

        visited.insert(from.to_string());
        self.find_neignbours(from)
            .iter()
            .filter(|n| !visited.contains(*n))
            .map(|n| self.path(n, destination, &mut visited.clone()))
            .max()
            .unwrap_or(0)
    }

    fn find_neignbours(&self, from: &str) -> HashSet<String> {
        let mut n = HashSet::<String>::new();
        //Children
        match self.orbits.get(from) {
            Some(children) => children.iter().for_each(|c| {
                n.insert(c.to_string());
            }),
            None => (),
        }
        //Parents
        self.orbits
            .iter()
            .filter(|(_, v)| v.contains(from))
            .for_each(|(k, _)| {
                n.insert(k.to_string());
            });
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let orbits = parse_input(&SAMPLE_DATA);
        assert_eq!(42, part1(&orbits));
    }
    #[test]
    fn test_part2() {
        let orbits = parse_input(&SAMPLE_2);
        assert_eq!(4, part2(&orbits));
    }
    lazy_static! {
        static ref SAMPLE_2: String = [
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN"
        ]
        .join("\n");
        static ref SAMPLE_DATA: String =
            ["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"]
                .join("\n");
    }
}
