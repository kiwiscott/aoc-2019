use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> (u32, u32) {
    let mut result = input.lines().next().unwrap().splitn(2, '-');
    let min = result.next().unwrap().parse::<u32>().unwrap();
    let max = result.next().unwrap().parse::<u32>().unwrap();
    (min, max)
}
#[aoc(day4, part1)]
fn part1((min, max): &(u32, u32)) -> usize {
    let include_groups = false;
    (*min..=*max)
        .into_iter()
        .filter(|p| is_valid(p, include_groups))
        .count()
}

#[aoc(day4, part2)]
fn part2((min, max): &(u32, u32)) -> usize {
    let include_groups = true;
    (*min..=*max)
        .into_iter()
        .filter(|p| is_valid(p, include_groups))
        .count()
}

fn is_valid(pass: &u32, include_groups: bool) -> bool {
    let p_chars: Vec<_> = pass.to_string().chars().collect();

    let mut chars_three_or_more = HashSet::<char>::new();
    if include_groups {
        let mut length = 0;
        let mut last_char = '_';

        for c in p_chars.iter() {
            if &last_char == c {
                length += 1;
                if length == 3 {
                    chars_three_or_more.insert(*c);
                }
            } else {
                length = 1;
                last_char = *c;
            }
        }
    }
    //It is a six-digit number.
    p_chars.len() == 6
        && (0..5)
            .into_iter()
            .any(|i| (!chars_three_or_more.contains(&p_chars[i])) && p_chars[i] == p_chars[i + 1])
        && (0..5).into_iter().all(|i| p_chars[i] <= p_chars[i + 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(true, is_valid(&111111_u32, false));
        assert_eq!(false, is_valid(&223450_u32, false));
        assert_eq!(false, is_valid(&123789_u32, false));
    }

    #[test]
    fn test_is_valid_part_2() {
        assert_eq!(true, is_valid(&112233_u32, true));
        assert_eq!(false, is_valid(&123444_u32, true));
        assert_eq!(true, is_valid(&111122_u32, true));
    }
}
