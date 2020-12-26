use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<f64> {
    input
        .lines()
        .map(|line| line.parse::<f64>().unwrap())
        .collect()
}
#[aoc(day1, part1)]
fn part1(numbers: &[f64]) -> i32 {
    numbers
        .iter()
        .map(|n| ((n / 3.0).floor() - 2.0) as i32)
        .sum()
}

#[aoc(day1, part2)]
fn part2(numbers: &[f64]) -> i32 {
    numbers.iter().map(|n| recursive_fuel(*n as i32)).sum()
}

fn recursive_fuel(mass: i32) -> i32 {
    let fuel = ((mass as f64 / 3.0).floor() - 2.0) as i32;
    if fuel < 0 {
        return 0;
    } else if fuel < 3 {
        return fuel;
    }

    fuel + recursive_fuel(fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = parse_input(&SAMPLE_DATA);
        assert_eq!(34241, part1(&data));
    }

    #[test]
    fn rec_part2() {
        assert_eq!(966, recursive_fuel(1969));
        for (module, fuel) in [(12, 2), (14, 2), (1969, 966), (100756, 50346)].iter() {
            assert_eq!(fuel, &recursive_fuel(*module));
        }
    }

    #[test]
    fn test_samples() {
        for (module, fuel) in [(12, 2), (14, 2), (1969, 654), (100756, 33583)].iter() {
            let numbers = vec![*module as f64];
            assert_eq!(fuel, &part1(&numbers));
        }
    }
    lazy_static! {
        static ref SAMPLE_DATA: String = ["12", "14", "1969", "100756", ""].join("\n");
    }
}
