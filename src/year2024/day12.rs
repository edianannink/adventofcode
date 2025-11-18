use crate::year2024::examples;

const DIRECTIONS: &[(isize, isize)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug)]
struct Group {
    #[allow(dead_code)]
    ch: char,
    members: Vec<(isize, isize)>,
}

impl Group {
    fn new(ch: char, members: Vec<(isize, isize)>) -> Self {
        Self { ch, members }
    }
}

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/input/day12.txt")
        .unwrap_or_else(|_| examples::DAY12.to_string());

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (part1, groups) = price_part1(&map);
    let part2 = price_part2(groups);

    (part1, part2)
}

fn price_part1(map: &[Vec<char>]) -> (usize, Vec<Group>) {
    let (rows, cols) = (map.len(), map[0].len());
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    let mut price = 0;

    let mut groups = Vec::new();

    for r in 0..rows {
        for c in 0..cols {
            if visited[r][c] {
                continue;
            }

            let ch = map[r][c];
            let mut members = Vec::new();
            let mut perimeter = Vec::new();

            visited[r][c] = true;

            let mut stack = vec![(r, c)];
            while let Some((cr, cc)) = stack.pop() {
                members.push((cr as isize, cc as isize));

                for (dr, dc) in DIRECTIONS {
                    let (nr, nc) = (cr as isize + *dr, cc as isize + *dc);
                    if nr < 0
                        || nr >= rows as isize
                        || nc < 0
                        || nc >= cols as isize
                        || map[nr as usize][nc as usize] != ch
                    {
                        perimeter.push((nr, nc));
                    }
                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if !visited[nr][nc] && map[nr][nc] == ch {
                            visited[nr][nc] = true;
                            stack.push((nr, nc));
                        }
                    }
                }
            }

            price += members.len() * perimeter.len();
            groups.push(Group::new(ch, members));
        }
    }

    (price, groups)
}

fn price_part2(groups: Vec<Group>) -> usize {
    let mut price = 0;

    for group in groups {
        let mut corners = 0;

        let min_r = group.members.iter().map(|(r, _)| r).min().unwrap();
        let max_r = group.members.iter().map(|(r, _)| r).max().unwrap();
        let min_c = group.members.iter().map(|(_, c)| c).min().unwrap();
        let max_c = group.members.iter().map(|(_, c)| c).max().unwrap();

        let rows = (max_r - min_r + 3) as usize;
        let cols = (max_c - min_c + 3) as usize;
        let mut matrix = vec![vec![0; cols]; rows];

        for &(r, c) in &group.members {
            matrix[(r - min_r + 1) as usize][(c - min_c + 1) as usize] = 1;
        }

        for r in 0..rows - 1 {
            for c in 0..cols - 1 {
                let sum = matrix[r][c] + matrix[r][c + 1] + matrix[r + 1][c] + matrix[r + 1][c + 1];
                match sum {
                    1 | 3 => {
                        corners += 1;
                    }
                    2 if matrix[r][c] == matrix[r + 1][c + 1] => {
                        corners += 2;
                    }
                    _ => {}
                }
            }
        }

        price += group.members.len() * corners;
    }

    price
}
