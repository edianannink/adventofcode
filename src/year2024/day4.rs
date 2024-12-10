use crate::year2024::examples;
use std::collections::HashMap;

const SEARCH_PATTERN: &str = "XMAS";

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2024/input/day4.txt")
        .unwrap_or_else(|_| examples::DAY4.to_string());

    let mut matrix = construct_matrix(input);

    (total_matches(&mut matrix), cross_mas_matches(&matrix))
}

pub(crate) fn construct_matrix(input: String) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();

    // Matrix of characters
    input.lines().for_each(|line| {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    });
    matrix
}

fn total_matches(matrix: &mut [Vec<char>]) -> usize {
    diagonal_matches(matrix)
        + diagonal_matches(
            &matrix
                .iter()
                .map(|row| row.iter().rev().cloned().collect())
                .collect::<Vec<Vec<char>>>(),
        )
        + row_matches(matrix)
        + col_matches(matrix)
}

fn transpose_matrix(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut diagonals: HashMap<usize, Vec<char>> = HashMap::new();

    // Diagonal traverse
    for (row, _) in matrix.iter().enumerate() {
        for (col, _) in matrix[0].iter().enumerate() {
            diagonals
                .entry(row + col)
                .or_default()
                .push(matrix[row][col]);
        }
    }

    // Collect the diagonals in order of their keys
    let mut result: Vec<Vec<char>> = vec![];
    let mut keys: Vec<usize> = diagonals.keys().cloned().collect();
    keys.sort(); // Ensure the keys are sorted

    for key in keys {
        if let Some(diagonal) = diagonals.get(&key) {
            result.push(diagonal.clone());
        }
    }

    result
}

fn diagonal_matches(matrix: &[Vec<char>]) -> usize {
    let transposed_matrix = transpose_matrix(matrix);

    // Number of matches in diagonals and reverse diagonals
    let mut matches = 0;
    for diagonal in &transposed_matrix {
        let string = diagonal.iter().collect::<String>();
        matches += num_matches(&string);
    }

    matches
}

fn row_matches(matrix: &[Vec<char>]) -> usize {
    let mut matches = 0;
    for row in matrix {
        let string = row.iter().collect::<String>();
        matches += num_matches(&string);
    }
    matches
}

fn col_matches(matrix: &[Vec<char>]) -> usize {
    let mut matches = 0;
    for col in 0..matrix[0].len() {
        let string: String = matrix.iter().map(|row| row[col]).collect();
        matches += num_matches(&string);
    }
    matches
}

fn num_matches(str: &str) -> usize {
    str.matches(SEARCH_PATTERN).count()
        + str
            .chars()
            .rev()
            .collect::<String>()
            .matches(SEARCH_PATTERN)
            .count()
}

fn cross_mas_matches(matrix: &[Vec<char>]) -> usize {
    let mut matches = 0;
    // Iterate over three rows at a time
    for i in 0..matrix.len() - 2 {
        let (top, middle, bottom) = (&matrix[i], &matrix[i + 1], &matrix[i + 2]);

        // If 'A' is found in middle, check top and bottom rows for all 4 orientations
        for col in 1..middle.len() - 1 {
            if middle[col] == 'A' {
                let (a, b, c, d) = (top[col - 1], top[col + 1], bottom[col - 1], bottom[col + 1]);

                if (a, b, c, d) == ('M', 'S', 'M', 'S')
                    || (a, b, c, d) == ('M', 'M', 'S', 'S')
                    || (a, b, c, d) == ('S', 'S', 'M', 'M')
                    || (a, b, c, d) == ('S', 'M', 'S', 'M')
                {
                    matches += 1;
                }
            }
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4() {
        let (part1, part2) = (18, 9);
        let mut matrix = construct_matrix(examples::DAY4.to_string());
        assert_eq!(total_matches(&mut matrix), part1);
        matrix = construct_matrix(examples::DAY4.to_string());
        assert_eq!(cross_mas_matches(&matrix), part2);
    }
}
