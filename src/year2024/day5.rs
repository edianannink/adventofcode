use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./input/day5.txt");

pub fn print_solution() {
    let (rules, pages) = INPUT.split_once("\n\n").unwrap();
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
        if p.is_sorted_by(|a, b| orderings[b].contains(a)) {
            p1 += p[p.len() / 2];
        } else {
            p.sort_by(|a, b| orderings[b].contains(a).cmp(&true));
            p2 += p[p.len() / 2];
        }
    }
    println!("Sum of middle page numbers of correct pages: {}", p1);
    println!("Sum of corrected pages: {}", p2);
}
