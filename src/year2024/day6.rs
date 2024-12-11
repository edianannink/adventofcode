use crate::year2024::day4::construct_matrix;
use crate::year2024::examples;
use std::collections::HashSet;

const OBSTACLE: char = '#';
const GUARD: &str = "<>^v";

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2024/input/day6.txt")
        .unwrap_or_else(|_| examples::DAY6.to_string());

    let mut matrix = construct_matrix(input.to_string());

    (part1(&matrix), part2(&mut matrix))
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn next_position(mut self, direction: Directions) -> Self {
        match direction {
            Directions::Up => self.y -= 1,
            Directions::Down => self.y += 1,
            Directions::Left => self.x -= 1,
            Directions::Right => self.x += 1,
        }
        self
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl From<&char> for Directions {
    fn from(value: &char) -> Self {
        match value {
            '^' => Directions::Up,
            'v' => Directions::Down,
            '<' => Directions::Left,
            '>' => Directions::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

impl Directions {
    fn rotate_right(&self) -> Self {
        match self {
            Directions::Up => Directions::Right,
            Directions::Down => Directions::Left,
            Directions::Left => Directions::Up,
            Directions::Right => Directions::Down,
        }
    }
}

fn guard_info(matrix: &[Vec<char>]) -> (Position, Directions) {
    // First we need to find the starting position and direction of the guard
    for (row, line) in matrix.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if GUARD.contains(*c) {
                return (Position::new(col, row), Directions::from(c));
            }
        }
    }
    panic!("Matrix should contain a guard");
}

fn get_char(matrix: &[Vec<char>], position: &Position) -> Option<char> {
    matrix
        .get(position.y)
        .and_then(|row| row.get(position.x))
        .cloned()
}

//TODO: Introduce Guard object that contains position and directions
fn traverse(matrix: &[Vec<char>], position: Position, directions: Directions) -> Option<usize> {
    let mut visited_pos_dir = HashSet::new();
    let mut visited = HashSet::new();
    let mut dir = directions.clone();
    let mut pos = position.clone();

    while pos.x > 0 && pos.x < matrix[0].len() - 1 && pos.y > 0 && pos.y < matrix.len() - 1 {
        if !visited_pos_dir.insert((pos.clone(), dir.clone())) {
            return None; // Loop detected
        }

        visited.insert(pos.clone());

        if let Some(OBSTACLE) = get_char(matrix, &pos.clone().next_position(dir.clone())) {
            if let Some(OBSTACLE) = get_char(matrix, &pos.clone().next_position(dir.rotate_right()))
            {
                dir = dir.rotate_right().rotate_right();
            } else {
                dir = dir.rotate_right();
            }
        }
        pos = pos.next_position(dir.clone());
    }

    visited.insert(pos.clone());

    Some(visited.len())
}

fn part1(matrix: &[Vec<char>]) -> usize {
    let (position, directions) = guard_info(matrix);

    // Now we need to traverse the matrix
    traverse(matrix, position, directions).expect("Part 1 should not contain any loops")
}

fn part2(matrix: &mut [Vec<char>]) -> usize {
    use rayon::prelude::*;

    let (position, directions) = guard_info(matrix);

    let rows = matrix.len();
    let cols = matrix[0].len();

    // Clone the matrix to allow parallel operations
    let matrix_clone: Vec<Vec<char>> = matrix.to_vec();

    // TODO: Brute force for now. Maybe think of a more efficient algorithm to do this
    (0..rows)
        .into_par_iter()
        .map(|row| {
            let mut detected_loops = 0;

            for col in 0..cols {
                let mut local_matrix = matrix_clone.clone(); // Create a local copy of the matrix
                let current_char = local_matrix[row][col];

                // Set obstacle if it is not already an obstacle
                if !GUARD.contains(current_char) {
                    local_matrix[row][col] = OBSTACLE;

                    if traverse(&local_matrix, position.clone(), directions.clone()).is_none() {
                        detected_loops += 1;
                    }
                }
            }

            detected_loops
        })
        .sum() // Combine results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1() {
        let matrix = construct_matrix(examples::DAY6.to_string());
        assert_eq!(part1(&matrix), 41);
    }

    #[test]
    fn test_day6_part2() {
        let mut matrix = construct_matrix(examples::DAY6.to_string());
        assert_eq!(part2(&mut matrix), 6);
    }
}
