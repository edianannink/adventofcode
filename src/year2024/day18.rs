use crate::year2024::examples;
use std::collections::{HashSet, VecDeque};

const DIRECTIONS: &[(isize, isize)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
const BYTES: usize = 12;

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/year2024/input/day18.txt")
        .unwrap_or_else(|_| examples::DAY18.to_string());

    let (map, additional) = parse_input(&input);

    (bfs(&map).to_string(), iterate(&map, &additional))
}

fn iterate(map: &[Vec<bool>], additional: &[(usize, usize)]) -> String {
    let mut map = map.to_owned();

    for &(add_r, add_c) in additional {
        map[add_r][add_c] = true;
        if bfs(&map) == 0 {
            return format!("{add_c},{add_r}");
        }
    }

    String::new()
}

fn bfs(map: &[Vec<bool>]) -> usize {
    let (rows, cols) = (map.len(), map[0].len());

    let mut queue = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back((0, 0, 0));
    visited.insert((0, 0));

    while let Some((cr, cc, cost)) = queue.pop_front() {
        if (cr, cc) == (rows - 1, cols - 1) {
            return cost;
        }

        for (dr, dc) in DIRECTIONS {
            let (nr, nc) = (cr as isize + *dr, cc as isize + *dc);
            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if visited.insert((nr, nc)) && !map[nr][nc] {
                    queue.push_back((nr, nc, cost + 1));
                }
            }
        }
    }

    0
}

fn parse_input(input: &str) -> (Vec<Vec<bool>>, Vec<(usize, usize)>) {
    let limit = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    let mut map = vec![vec![false; limit + 1]; limit + 1];
    let mut additional = Vec::new();
    for (byte, line) in input.lines().enumerate() {
        let parts = line
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if byte < BYTES {
            map[parts[1]][parts[0]] = true;
        } else {
            additional.push((parts[1], parts[0]));
        }
    }
    (map, additional)
}
