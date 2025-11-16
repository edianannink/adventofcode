use crate::year2024::examples;

const DIRECTIONS: &[(isize, isize)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/input/day12.txt")
        .unwrap_or_else(|_| examples::DAY12.to_string());

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    (price_part1(&map), 0)
}

fn price_part1(map: &[Vec<char>]) -> usize {
    let (rows, cols) = (map.len(), map[0].len());
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    let mut price = 0;

    for r in 0..rows {
        for c in 0..cols {
            if visited[r][c] {
                continue;
            }

            let group = map[r][c];
            let mut area = 0;
            let mut perimeter = 0;

            visited[r][c] = true;

            let mut stack = vec![(r, c)];
            while let Some((cr, cc)) = stack.pop() {
                area += 1;

                for (dr, dc) in DIRECTIONS {
                    let (nr, nc) = (cr as isize + *dr, cc as isize + *dc);
                    if nr < 0
                        || nr >= rows as isize
                        || nc < 0
                        || nc >= cols as isize
                        || map[nr as usize][nc as usize] != group
                    {
                        perimeter += 1;
                    }
                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if !visited[nr][nc] && map[nr][nc] == group {
                            visited[nr][nc] = true;
                            stack.push((nr, nc));
                        }
                    }
                }
            }

            price += area * perimeter;
        }
    }

    price
}
