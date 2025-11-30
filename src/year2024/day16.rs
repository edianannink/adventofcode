use crate::year2024::examples;
use std::{cmp::Reverse, collections::BinaryHeap};

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/input/day16.txt")
        .unwrap_or_else(|_| examples::DAY16.to_string());

    (part1(&input), 0)
}

pub fn part1(input: &str) -> usize {
    let maze: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (r, row) in maze.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (r, c);
            } else if cell == 'E' {
                end = (r, c);
            }
        }
    }

    dijkstra(&maze, start, end)
}

fn dijkstra(maze: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> usize {
    let (rows, cols) = (maze.len(), maze[0].len());
    let mut heap = BinaryHeap::new();
    let mut dist = vec![vec![vec![usize::MAX; 4]; cols]; rows];

    dist[start.0][start.1][1] = 0;
    heap.push(Reverse((0, start.0, start.1, 1)));

    while let Some(Reverse((cost, cr, cc, dir))) = heap.pop() {
        if (cr, cc) == end {
            return cost;
        }

        if cost > dist[cr][cc][dir] {
            continue;
        }

        let (dr, dc) = DIRECTIONS[dir];
        let (nr, nc) = (cr as isize + dr, cc as isize + dc);

        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
            let (nr, nc) = (nr as usize, nc as usize);

            if maze[nr][nc] != '#' {
                let new_cost = cost + 1;
                if new_cost < dist[nr][nc][dir] {
                    dist[nr][nc][dir] = new_cost;
                    heap.push(Reverse((new_cost, nr, nc, dir)));
                }
            }
        }

        for &new_dir in &[(dir + 3) % 4, (dir + 1) % 4] {
            let new_cost = cost + 1000;
            if new_cost < dist[cr][cc][new_dir] {
                dist[cr][cc][new_dir] = new_cost;
                heap.push(Reverse((new_cost, cr, cc, new_dir)));
            }
        }
    }

    0
}
