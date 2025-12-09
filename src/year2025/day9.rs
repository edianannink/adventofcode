use crate::year2025::examples;
use std::collections::HashSet;

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/year2025/input/day9.txt")
        .unwrap_or_else(|_| examples::DAY9.to_string());

    let tiles: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line
                .split(',')
                .map(|num| num.trim().parse().unwrap())
                .collect();
            (parts[1], parts[0])
        })
        .collect();

    (part1(&tiles).to_string(), part2(&tiles).to_string())
}

fn part1(tiles: &[(usize, usize)]) -> usize {
    let size = tiles.len();
    let mut max_area = 0;
    for i in 0..size {
        for j in (i + 1)..size {
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
    let size = tiles.len();
    let rows = tiles.iter().map(|&(r, _)| r).max().unwrap() + 1;
    let cols = tiles.iter().map(|&(_, c)| c).max().unwrap() + 1;

    let polygon = polygon(tiles);
    let mut is_polygon = vec![vec![false; cols]; rows];
    for (r, c) in polygon {
        is_polygon[r][c] = true;
    }

    let mut max_area = 0;

    for i in 0..size {
        for j in (i + 1)..size {
            let (row1, col1) = tiles[i];
            let (row2, col2) = tiles[j];

            let width = col1.abs_diff(col2) + 1;
            let height = row1.abs_diff(row2) + 1;

            let area = width * height;
            if area < max_area {
                continue;
            }

            // Skip if the polygon is crossing the inner edge of the rectangle
            if width > 2 && height > 2 {
                let min_row = row1.min(row2);
                let max_row = row1.max(row2);
                let min_col = col1.min(col2);
                let max_col = col1.max(col2);

                if (min_col + 1..max_col)
                    .any(|c| is_polygon[min_row + 1][c] || is_polygon[max_row - 1][c])
                {
                    continue;
                }

                if (min_row + 1..max_row)
                    .any(|r| is_polygon[r][min_col + 1] || is_polygon[r][max_col - 1])
                {
                    continue;
                }
            }

            max_area = max_area.max(area);
        }
    }

    max_area
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
