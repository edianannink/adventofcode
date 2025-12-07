use crate::year2024::examples;
use regex::Regex;

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/input/day13.txt")
        .unwrap_or_else(|_| examples::DAY13.to_string());

    let parsed_input = parse_input(&input);

    (
        part1(&parsed_input).to_string(),
        part2(&parsed_input).to_string(),
    )
}

fn part1(parsed_input: &[isize]) -> usize {
    parsed_input
        .chunks(6)
        .filter_map(|chunk| solve(chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5]))
        .sum()
}

fn part2(parsed_input: &[isize]) -> usize {
    const OFFSET: isize = 10000000000000;
    parsed_input
        .chunks(6)
        .filter_map(|chunk| {
            solve(
                chunk[0],
                chunk[1],
                chunk[2],
                chunk[3],
                chunk[4] + OFFSET,
                chunk[5] + OFFSET,
            )
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<isize> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(input)
        .map(|m| m.as_str().parse::<isize>().unwrap())
        .collect()
}

fn solve(ax: isize, ay: isize, bx: isize, by: isize, x: isize, y: isize) -> Option<usize> {
    let det = ax * by - ay * bx;
    if det == 0 {
        return None;
    }

    let num_a = x * by - y * bx;
    let num_b = ax * y - ay * x;
    if num_a % det != 0 || num_b % det != 0 {
        return None;
    }

    let a = num_a / det;
    let b = num_b / det;
    if a < 0 || b < 0 {
        return None;
    }

    Some((3 * a + b) as usize)
}
