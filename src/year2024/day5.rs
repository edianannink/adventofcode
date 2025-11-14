use crate::year2024::examples;
use std::collections::{HashMap, HashSet};

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2024/input/day5.txt")
        .unwrap_or_else(|_| examples::DAY5.to_string());

    compute_pages(input)
}

fn compute_pages(input: String) -> (usize, usize) {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let mut orderings = HashMap::<usize, HashSet<usize>>::new();

    for l in rules.lines() {
        let (x, y) = l.split_once('|').unwrap();
        orderings
            .entry(y.parse().unwrap())
            .or_default()
            .insert(x.parse().unwrap());
    }
    let pages = pages.lines().map(|l| {
        l.split(',')
            .map(|w| w.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    let (mut p1, mut p2) = (0, 0);
    for mut p in pages {
        if p.is_sorted_by(|a, b| orderings.get(b).is_some_and(|set| set.contains(a))) {
            p1 += p[p.len() / 2];
        } else {
            p.sort_by(|a, b| {
                orderings
                    .get(a)
                    .map_or(false.cmp(&true), |set| set.contains(b).cmp(&true))
            });
            p2 += p[p.len() / 2];
        }
    }
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5() {
        let (part1, part2) = (143, 123);
        assert_eq!(compute_pages(examples::DAY5.to_string()), (part1, part2));
    }
}
