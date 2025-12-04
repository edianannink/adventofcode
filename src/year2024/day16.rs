use crate::year2024::examples;
use std::collections::{HashSet, VecDeque};
use std::{cmp::Reverse, collections::BinaryHeap};

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2024/input/day16.txt")
        .unwrap_or_else(|_| examples::DAY16_1.to_string());

    solve(&input)
}

pub fn solve(input: &str) -> (usize, usize) {
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

fn dijkstra(maze: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> (usize, usize) {
    let (rows, cols) = (maze.len(), maze[0].len());
    let mut heap = BinaryHeap::new();
    let mut dist = vec![vec![vec![usize::MAX; 4]; cols]; rows];
    let mut predecessors: Vec<Vec<Vec<Vec<(usize, usize, usize)>>>> =
        vec![vec![vec![Vec::new(); 4]; cols]; rows];

    dist[start.0][start.1][1] = 0;
    heap.push(Reverse((0, start.0, start.1, 1)));

    while let Some(Reverse((cost, cr, cc, dir))) = heap.pop() {
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
                    predecessors[nr][nc][dir] = vec![(cr, cc, dir)];
                    heap.push(Reverse((new_cost, nr, nc, dir)));
                } else if new_cost == dist[nr][nc][dir] {
                    predecessors[nr][nc][dir].push((cr, cc, dir));
                }
            }
        }

        for &new_dir in &[(dir + 3) % 4, (dir + 1) % 4] {
            let new_cost = cost + 1000;
            if new_cost < dist[cr][cc][new_dir] {
                dist[cr][cc][new_dir] = new_cost;
                predecessors[cr][cc][new_dir] = vec![(cr, cc, dir)];
                heap.push(Reverse((new_cost, cr, cc, new_dir)));
            } else if new_cost == dist[cr][cc][new_dir] {
                predecessors[cr][cc][new_dir].push((cr, cc, dir));
            }
        }
    }

    let min_cost = (0..DIRECTIONS.len())
        .map(|d| dist[end.0][end.1][d])
        .min()
        .unwrap();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut tiles = HashSet::new();

    (0..DIRECTIONS.len()).for_each(|dir| {
        if dist[end.0][end.1][dir] == min_cost {
            queue.push_back((end.0, end.1, dir));
            visited.insert((end.0, end.1, dir));
        }
    });

    while let Some((r, c, dir)) = queue.pop_front() {
        tiles.insert((r, c));

        for &(pr, pc, pdir) in &predecessors[r][c][dir] {
            if visited.insert((pr, pc, pdir)) {
                queue.push_back((pr, pc, pdir));
            }
        }
    }

    (min_cost, tiles.len())
}
