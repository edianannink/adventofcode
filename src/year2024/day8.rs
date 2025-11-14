use crate::year2024::examples;
use std::collections::{HashMap, HashSet};

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/input/day8.txt")
        .unwrap_or_else(|_| examples::DAY8.to_string());

    (antinodes_part1(&input), antinodes_part2(&input))
}

fn antinodes_part1(input: &str) -> usize {
    let mut antinodes = HashSet::new();
    let input_matrix = ascii_matrix(input);

    let pairs = find_pairs(input);

    let rows = input_matrix.len() as isize;
    let cols = input_matrix[0].len() as isize;

    for positions in pairs.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];

                let dr = r2 - r1;
                let dc = c2 - c1;

                let antinode1 = (r1 - dr, c1 - dc);
                let antinode2 = (r2 + dr, c2 + dc);

                for &(r, c) in &[antinode1, antinode2] {
                    if r >= 0 && r < rows && c >= 0 && c < cols {
                        antinodes.insert((r, c));
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn antinodes_part2(input: &str) -> usize {
    let mut antinodes = HashSet::new();
    let mut grid = ascii_matrix(input);

    let pairs = find_pairs(input);
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for positions in pairs.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];

                let dr = r2 - r1;
                let dc = c2 - c1;

                let antinode1 = (r1 - dr, c1 - dc);
                let antinode2 = (r2 + dr, c2 + dc);

                let mut stack = vec![antinode1];
                while let Some((row_pos, col_pos)) = stack.pop() {
                    if row_pos < 0 || row_pos >= rows || col_pos < 0 || col_pos >= cols {
                        break;
                    }

                    if grid[row_pos as usize][col_pos as usize] != '#'
                        && !grid[row_pos as usize][col_pos as usize].is_ascii_alphanumeric()
                    {
                        grid[row_pos as usize][col_pos as usize] = '#';
                        antinodes.insert((row_pos, col_pos));
                    }

                    stack.push((row_pos - dr, col_pos - dc));
                }

                let mut stack = vec![antinode2];
                while let Some((row_pos, col_pos)) = stack.pop() {
                    if row_pos <= 0 || row_pos >= rows || col_pos < 0 || col_pos >= cols {
                        break;
                    }

                    if grid[row_pos as usize][col_pos as usize] != '#'
                        && !grid[row_pos as usize][col_pos as usize].is_ascii_alphanumeric()
                    {
                        grid[row_pos as usize][col_pos as usize] = '#';
                        antinodes.insert((row_pos, col_pos));
                    }

                    stack.push((row_pos + dr, col_pos + dc));
                }

                antinodes.insert(positions[i]);
                antinodes.insert(positions[j]);
            }
        }
    }

    antinodes.len()
}

fn find_pairs(input: &str) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antenna_pairs: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (row, antenna) in input.lines().enumerate() {
        for (col, c) in antenna.chars().enumerate() {
            if c.is_ascii_alphanumeric() {
                antenna_pairs
                    .entry(c)
                    .or_default()
                    .push((row as isize, col as isize));
            }
        }
    }

    antenna_pairs
}

fn ascii_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        matrix.push(row);
    }
    matrix
}
