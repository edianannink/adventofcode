use crate::year2025::examples;

const DIRECTIONS: &[(isize, isize)] = &[
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2025/input/day4.txt")
        .unwrap_or_else(|_| examples::DAY4.to_string());

    (part1(&input), part2(&input))
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (map.len(), map[0].len());

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] != '@' {
                continue;
            };
            if count_rolls(&map, rows, cols, r, c) < 4 {
                result += 1
            };
        }
    }
    result
}

fn part2(input: &str) -> usize {
    let mut result = 0;
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (map.len(), map[0].len());

    loop {
        let mut removed = 0;
        for r in 0..rows {
            for c in 0..cols {
                if map[r][c] != '@' {
                    continue;
                };
                if count_rolls(&map, rows, cols, r, c) < 4 {
                    map[r][c] = 'x';
                    removed += 1;
                };
            }
        }
        if removed == 0 {
            break;
        } else {
            result += removed;
        }
    }
    result
}

fn count_rolls(map: &[Vec<char>], rows: usize, cols: usize, r: usize, c: usize) -> usize {
    let mut rolls = 0;
    for (dr, dc) in DIRECTIONS {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
            let (nr, nc) = (nr as usize, nc as usize);
            if map[nr][nc] == '@' {
                rolls += 1;
            }
        }
    }
    rolls
}
