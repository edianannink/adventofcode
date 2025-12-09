use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};
use crate::year2025::examples;

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/year2025/input/day9.txt")
        .unwrap_or_else(|_| examples::DAY9.to_string());

    let tiles: Vec<(usize, usize)> = input.lines().map(|line| {
        let parts: Vec<usize> = line
            .split(',')
            .map(|num| num.trim().parse().unwrap())
            .collect();
        (parts[1], parts[0])
    }).collect();

    (part1(&tiles).to_string(), part2(&tiles).to_string())
}

fn part1(tiles: &[(usize, usize)]) -> usize {
    let size = tiles.len();
    let mut max_area = 0;
    for i in 0..size {
        for j in 0..size {
            let (row1, col1) = tiles[i];
            let (row2, col2) = tiles[j];

            let width = col1.abs_diff(col2) + 1;
            let height = row1.abs_diff(row2) + 1;
            let area = width * height;

            max_area = max_area.max(area);
        }
    }
    max_area
}

fn part2(tiles: &[(usize, usize)]) -> usize {
    let polygon = polygon(tiles);
    let interior = flood_fill(tiles, &polygon);
    // for r in 0..8 {
    //     for c in 0..12 {
    //         if interior.contains(&(r, c)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    0
}

fn flood_fill(tiles: &[(usize, usize)], polygon: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    let rows = tiles.iter().map(|&(r, _)| r).max().unwrap() + 1;
    let cols = tiles.iter().map(|&(_, c)| c).max().unwrap() + 1;

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0,0));
    visited.insert((0,0));

    while let Some((cr, cc)) = queue.pop_front() {
        for (dr, dc) in DIRECTIONS {
            let nr = cr as isize + dr;
            let nc = cc as isize + dc;
            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if !polygon.contains(&(nr, nc)) && visited.insert((nr, nc)) {
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    let mut interior = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if !visited.contains(&(r, c)) {
                interior.push((r, c));
            }
        }
    }

    interior
}

fn polygon(tiles: &[(usize, usize)]) -> HashSet<(usize, usize)> {
    let mut polygon = HashSet::new();
    for i in 0..tiles.len() {
        let next = (i + 1) % tiles.len();
        let (r1, c1) = tiles[i];
        let (r2, c2) = tiles[next];

        if r1 == r2 {
            for c in c1.min(c2)..=c1.max(c2) {
                polygon.insert((r1, c));
            }
        } else {
            for r in r1.min(r2)..=r1.max(r2) {
                polygon.insert((r, c1));
            }
        }
    }
    polygon
}