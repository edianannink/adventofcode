use crate::year2024::examples;
use regex::Regex;
use std::ops::Mul;

const REGEX: &str = r#"(mul\((\d+),(\d+)\))"#;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2024/input/day3.txt")
        .unwrap_or_else(|_| examples::DAY3.to_string());

    (mul_sum(&input), do_mul(&input))
}

fn mul_sum(input_str: &str) -> usize {
    Regex::new(REGEX)
        .unwrap()
        .captures_iter(input_str)
        .map(|mat| {
            mat.get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap()
                .mul(mat.get(3).unwrap().as_str().parse::<usize>().unwrap())
        })
        .sum()
}

fn do_mul(input_str: &str) -> usize {
    let mut sum = 0;
    let mut start_pos = 0;

    while let Some(start) = input_str[start_pos..].find("do()") {
        let start_index = start_pos + start + "do()".len();
        let end_index = input_str[start_index..]
            .find("don't()")
            .map_or(input_str.len(), |end| start_index + end);

        sum += mul_sum(&input_str[start_index..end_index]);
        start_pos = end_index;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3() {
        let (part1, part2) = (161, 48);
        assert_eq!(mul_sum(examples::DAY3), part1);
        assert_eq!(do_mul(examples::DAY3), part2);
    }
}
