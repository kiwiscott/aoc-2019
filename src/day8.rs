use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let numbers: Vec<i32> = input
        .chars()
        .filter(|p| p.is_ascii_digit())
        .map(|m| m.to_string().parse::<i32>().unwrap())
        .collect();

    let mut result: Vec<Vec<i32>> = vec![];
    for i in numbers.chunks(HEIGHT * WIDTH) {
        result.push(i.to_vec());
    }
    result
}

const HEIGHT: usize = 25;
const WIDTH: usize = 6;

#[aoc(day8, part1)]
fn part1(layers: &Vec<Vec<i32>>) -> usize {
    let mut layered = layers.clone();

    layered.sort_by(|a, b| {
        a.iter()
            .filter(|p| **p == 0)
            .count()
            .cmp(&b.iter().filter(|p| **p == 0).count())
    });

    layered.first().unwrap().iter().filter(|p| **p == 1).count()
        * layered.first().unwrap().iter().filter(|p| **p == 2).count()
}

#[aoc(day8, part2)]
fn part2(layers: &Vec<Vec<i32>>) -> usize {
    let mut cleaned = vec!['#'; 150];
    for i in 0..(WIDTH * HEIGHT) {
        for r in layers {
            match r[i] {
                1 => {
                    break;
                }
                0 => {
                    cleaned[i] = ' ';
                    break;
                }
                _ => (),
            }
        }
    }

    for c in cleaned.chunks(25) {
        println!("{}", c.iter().map(|n| n.to_string()).collect::<String>());
    }
    println!("\n\n");

    0
}
