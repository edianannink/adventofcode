use std::collections::{HashSet, VecDeque};

use crate::year2025::examples;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2025/input/day7.txt")
        .unwrap_or_else(|_| examples::DAY7.to_string());

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&cell| cell == 'S').map(|c| (r, c)))
        .unwrap();

    (bfs(&map, start), dp(&map, start))
}

fn bfs(map: &[Vec<char>], start: (usize, usize)) -> usize {
    let rows = map.len();
    let mut splits = 0;

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(start);
    visited.insert(start);

    while let Some((cr, cc)) = queue.pop_front() {
        let nr = cr + 1;
        if nr < rows {
            if map[nr][cc] == '^' {
                splits += 1;
                for &dir in [(nr, cc - 1), (nr, cc + 1)].iter() {
                    if visited.insert(dir) {
                        queue.push_back(dir);
                    }
                }
            } else if visited.insert((nr, cc)) {
                queue.push_back((nr, cc));
            }
        }
    }

    splits
}

// DFS results in exponential time complexity, so we use DP instead
fn dp(map: &[Vec<char>], start: (usize, usize)) -> usize {
    let (rows, cols) = (map.len(), map[0].len());

    let mut paths = vec![vec![0usize; cols]; rows];
    paths[start.0][start.1] = 1;

    for r in start.0..rows {
        for c in 0..cols {
            if paths[r][c] == 0 {
                continue;
            }

            let nr = r + 1;
            if nr >= rows {
                continue;
            }

            if map[nr][c] == '^' {
                if c > 0 {
                    paths[nr][c - 1] += paths[r][c];
                }
                if c + 1 < cols {
                    paths[nr][c + 1] += paths[r][c];
                }
            } else {
                paths[nr][c] += paths[r][c];
            }
        }
    }

    paths[rows - 1].iter().sum()
}
