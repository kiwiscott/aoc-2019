use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect()
}

#[aoc(day16, part1)]
fn part1(numbers: &[i64]) -> String {
    let out_list = flawed_frequency_transmission(100, &numbers.to_vec());
    out_list.iter().fold(String::new(), |mut s, o| {
        s.push_str(&o.to_string());
        s
    })
}
#[aoc(day16, part2)]

fn part2(numbers: &Vec<i64>) -> String {
    let offset: usize = numbers[0..7]
        .iter()
        .fold(String::new(), |mut acc, n| {
            acc.push_str(&n.to_string());
            acc
        })
        .parse()
        .unwrap();

    let mut adj_input: Vec<i64> = numbers
        .iter()
        .cycle()
        .take(10_000 * numbers.len())
        .skip(offset)
        .map(|n| *n)
        .collect();

    // This pattern only works on the 2nd half of the array
    // But lo and behold, the offset is in the 2nd half
    for _ in 0..100 {
        for i in (0..adj_input.len() - 1).rev() {
            adj_input[i] = (adj_input[i] + adj_input[i + 1]) % 10;
        }
    }

    adj_input[0..8].iter().fold(String::new(), |mut s, o| {
        s.push_str(&o.to_string());
        s
    })
}

fn generate_pattern(base: &[i64], output_element: usize) -> Vec<i64> {
    let mut r = vec![];

    for b in base {
        for _ in 0..output_element {
            r.push(*b);
        }
    }
    r
}

fn flawed_frequency_transmission(phases: usize, numbers: &[i64]) -> Vec<i64> {
    let base_pattern: Vec<i64> = vec![0, 1, 0, -1];

    let mut res_vec = numbers.to_vec();

    for _ in 0..phases {
        let numbers = res_vec.clone();

        for out_element in 0..numbers.len() {
            let pattern_iter = generate_pattern(base_pattern.as_slice(), out_element + 1);

            let summed: i64 = numbers
                .iter()
                .zip(pattern_iter.iter().cycle().skip(1))
                .filter_map(|(num, mul)| {
                    //print!("{:?}*{:?} + ", num, mul);
                    if mul == &0 || num == &0 {
                        None
                    } else {
                        Some(num * mul)
                    }
                })
                .sum();
            //print!("out_element:{:?}== {:?}\n", out_element, summed);
            res_vec[out_element] = (summed % 10).abs();
        }
    }

    res_vec[0..8].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_sample_2() {
        let sam = String::from("03036732577212944063491565474664");
        let data = parse_input(&sam);

        let expected: Vec<i64> = vec![8, 4, 4, 6, 2, 0, 2, 6];
        assert_eq!(expected, flawed_frequency_transmission(100, &data));
    }

    //#[test]
    fn test_example_1() {
        let sam = String::from("80871224585914546619083218645595");
        let data = parse_input(&sam);

        let expected: Vec<i64> = vec![22, 4, 1, 7, 6, 1, 7, 6];
        assert_eq!(expected, flawed_frequency_transmission(100, &data));
    }

    #[test]
    fn test_sample_1() {
        let sam = String::from("12345678");
        let data = parse_input(&sam);

        let expected: Vec<i64> = vec![4, 8, 2, 2, 6, 1, 5, 8];
        assert_eq!(expected, flawed_frequency_transmission(1, &data));
    }
}
