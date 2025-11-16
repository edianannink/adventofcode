use crate::year2024::examples;
use std::collections::HashMap;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/input/day11.txt")
        .unwrap_or_else(|_| examples::DAY11.to_string());

    (blink_part1(&input), blink_part2(&input))
}

fn blink_part1(input: &str) -> usize {
    let mut stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    for _ in 0..25 {
        let mut new_stones = Vec::with_capacity(stones.len() * 2);
        for &stone in &stones {
            if stone == 0 {
                new_stones.push(1);
            } else {
                let stone_str = stone.to_string();
                let len = stone_str.len();
                if len % 2 == 0 {
                    let (left, right) = stone_str.split_at(len / 2);
                    new_stones.push(left.parse::<usize>().unwrap());
                    new_stones.push(right.parse::<usize>().unwrap());
                } else {
                    new_stones.push(stone * 2024);
                }
            }
        }
        stones = new_stones;
    }

    stones.len()
}

// HashMap optimization
fn blink_part2(input: &str) -> usize {
    let mut stone_counts: HashMap<usize, usize> = HashMap::new();
    for stone in input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
    {
        *stone_counts.entry(stone).or_default() += 1;
    }

    for _ in 0..75 {
        let mut new_counts: HashMap<usize, usize> = HashMap::new();
        for (&stone, &count) in stone_counts.iter() {
            if stone == 0 {
                *new_counts.entry(1).or_default() += count;
            } else {
                let stone_str = stone.to_string();
                let len = stone_str.len();
                if len % 2 == 0 {
                    let (left, right) = stone_str.split_at(len / 2);
                    let left_val = left.parse::<usize>().unwrap();
                    let right_val = right.parse::<usize>().unwrap();
                    *new_counts.entry(left_val).or_default() += count;
                    *new_counts.entry(right_val).or_default() += count;
                } else {
                    *new_counts.entry(stone * 2024).or_default() += count;
                }
            }
        }
        stone_counts = new_counts;
    }

    stone_counts.values().sum()
}
