use crate::year2024::examples;
use std::collections::HashSet;

const DIRECTIONS: &[(isize, isize)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/input/day10.txt")
        .unwrap_or_else(|_| examples::DAY10.to_string());

    let matrix: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    trails(matrix)
}

fn trails(input: Vec<Vec<usize>>) -> (usize, usize) {
    let (rows, cols) = (input.len(), input[0].len());

    let (mut sum_part1, mut sum_part2) = (0, 0);
    for row in 0..rows {
        for col in 0..cols {
            if input[row][col] == 9 {
                sum_part1 += explore_part1((row, col), &input).len();
                sum_part2 += explore_part2((row, col), &input).len();
            }
        }
    }
    (sum_part1, sum_part2)
}

fn explore_part1(start: (usize, usize), input: &[Vec<usize>]) -> HashSet<(usize, usize)> {
    let (rows, cols) = (input.len(), input[0].len());

    let mut stack: Vec<(usize, usize)> = vec![start];
    let mut visited: HashSet<(usize, usize)> = HashSet::from([start]);
    let mut result: HashSet<(usize, usize)> = HashSet::new();

    while let Some((r, c)) = stack.pop() {
        let height = input[r][c];
        if height == 0 {
            result.insert((r, c));
            continue;
        }

        for (dr, dc) in DIRECTIONS {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if input[nr][nc] + 1 == height && visited.insert((nr, nc)) {
                    stack.push((nr, nc));
                }
            }
        }
    }

    result
}

fn explore_part2(start: (usize, usize), input: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let (rows, cols) = (input.len(), input[0].len());

    let mut stack: Vec<(usize, usize)> = vec![start];
    let mut result = Vec::new();

    while let Some((r, c)) = stack.pop() {
        let height = input[r][c];
        if height == 0 {
            result.push((r, c));
            continue;
        }

        for (dr, dc) in DIRECTIONS {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if input[nr][nc] + 1 == height {
                    stack.push((nr, nc));
                }
            }
        }
    }

    result
}
