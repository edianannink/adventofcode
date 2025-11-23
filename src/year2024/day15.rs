use std::collections::HashSet;

use crate::year2024::examples;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/input/day15.txt")
        .unwrap_or_else(|_| examples::DAY15.to_string());

    let (start_pos, map, moves) = parse_input(&input);

    (
        part1(start_pos, map.clone(), &moves),
        part2(start_pos, map, &moves),
    )
}

fn part1(start_pos: (usize, usize), mut map: Vec<Vec<char>>, moves: &[char]) -> usize {
    let (mut r, mut c) = (start_pos.0 as isize, start_pos.1 as isize);
    let rows = map.len();
    let cols = map[0].len();

    for &dir in moves {
        let (dr, dc) = convert_dir(&dir);
        let mut path = Vec::new();
        let mut positions = Vec::new();
        let mut nr = r;
        let mut nc = c;

        while nr > 0 && nr < (rows as isize - 1) && nc > 0 && nc < (cols as isize - 1) {
            path.push(map[nr as usize][nc as usize]);
            positions.push((nr as usize, nc as usize));
            nr += dr;
            nc += dc;
        }

        let len = path.len();

        if len > 1 {
            match path[1] {
                '.' => {
                    r += dr;
                    c += dc;
                    path.swap(1, 0);
                }
                'O' => {
                    let mut i = 1;
                    while i < len && path[i] == 'O' {
                        i += 1;
                    }

                    if i >= len || path[i] != '.' {
                        continue;
                    }

                    for j in (0..i).rev() {
                        path.swap(j + 1, j);
                    }

                    r += dr;
                    c += dc;
                }
                _ => {}
            }

            for (i, (pr, pc)) in positions.iter().enumerate() {
                map[*pr][*pc] = path[i];
            }
        }
    }

    let mut sum = 0;

    for (r, chars) in map.iter().enumerate() {
        for (c, char) in chars.iter().enumerate() {
            if *char == 'O' {
                sum += 100 * r + c;
            }
        }
    }

    sum
}

fn part2(start_pos: (usize, usize), mut map: Vec<Vec<char>>, moves: &[char]) -> usize {
    let (mut r, mut c) = (start_pos.0 as isize, (start_pos.1 * 2) as isize);

    // Part 2 pre-processing
    map.iter_mut().for_each(|line| {
        *line = line
            .iter()
            .flat_map(|&ch| match ch {
                '#' => ['#', '#'],
                'O' => ['[', ']'],
                '.' => ['.', '.'],
                '@' => ['@', '.'],
                _ => panic!("Should not be possible!"),
            })
            .collect();
    });

    let rows = map.len();
    let cols = map[0].len();

    for &dir in moves {
        let (dr, dc) = convert_dir(&dir);
        let mut path = Vec::new();
        let mut positions = Vec::new();
        let mut nr = r;
        let mut nc = c;

        if dir == '<' || dir == '>' {
            while nr > 0 && nr < (rows as isize - 1) && nc > 0 && nc < (cols as isize - 1) {
                path.push(map[nr as usize][nc as usize]);
                positions.push((nr as usize, nc as usize));
                nr += dr;
                nc += dc;
            }

            let len = path.len();

            if len > 1 {
                match path[1] {
                    '.' => {
                        r += dr;
                        c += dc;
                        path.swap(1, 0);
                    }
                    '[' | ']' => {
                        let mut i = 1;
                        while i < len && (path[i] == '[' || path[i] == ']') {
                            i += 1;
                        }

                        if i >= len || path[i] != '.' {
                            continue;
                        }

                        for j in (0..i).rev() {
                            path[j + 1] = path[j]
                        }

                        path[0] = '.';

                        r += dr;
                        c += dc;
                    }
                    _ => {}
                }

                for (i, (pr, pc)) in positions.iter().enumerate() {
                    map[*pr][*pc] = path[i];
                }
            }
        } else {
            let neighbour = map[(r + dr) as usize][(c + dc) as usize];
            if neighbour == '.' {
                map[(r + dr) as usize][(c + dc) as usize] = '@';
                map[r as usize][c as usize] = '.';
                r += dr;
                c += dc;
            } else if neighbour == '[' || neighbour == ']' {
                let mut stack = vec![((r + dr) as usize, (c + dc) as usize)];
                let mut to_move = HashSet::new();
                let mut can_move = true;

                while let Some((sr, sc)) = stack.pop() {
                    if to_move.contains(&(sr, sc)) {
                        continue;
                    }

                    let cell = map[sr][sc];
                    if cell == '#' {
                        can_move = false;
                        break;
                    }
                    if cell == '.' {
                        continue;
                    }

                    let (left_c, right_c) = if cell == '[' {
                        (sc, sc + 1)
                    } else {
                        (sc - 1, sc)
                    };

                    to_move.insert((sr, left_c));
                    to_move.insert((sr, right_c));

                    let next_r = (sr as isize + dr) as usize;
                    stack.push((next_r, left_c));
                    stack.push((next_r, right_c));
                }

                if can_move {
                    let mut sorted: Vec<_> = to_move.iter().copied().collect();
                    sorted.sort_by_key(|(sr, _)| {
                        if dr > 0 {
                            -((*sr) as isize)
                        } else {
                            *sr as isize
                        }
                    });

                    let values: Vec<_> = sorted.iter().map(|(sr, sc)| map[*sr][*sc]).collect();
                    for &(sr, sc) in &sorted {
                        map[sr][sc] = '.';
                    }

                    for (i, &(sr, sc)) in sorted.iter().enumerate() {
                        let new_r = (sr as isize + dr) as usize;
                        map[new_r][sc] = values[i];
                    }

                    map[(r + dr) as usize][(c + dc) as usize] = '@';
                    map[r as usize][c as usize] = '.';
                    r += dr;
                    c += dc;
                }
            }
        }
    }

    let mut sum = 0;

    for (r, chars) in map.iter().enumerate() {
        for (c, char) in chars.iter().enumerate() {
            if *char == '[' {
                sum += 100 * r + c;
            }
        }
    }

    sum
}
fn convert_dir(char: &char) -> (isize, isize) {
    match char {
        '<' => (0, -1),
        '>' => (0, 1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => panic!("Should not be possible"),
    }
}

fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<char>>, Vec<char>) {
    let mut parts = input.split("\n\n");
    let mut start_pos = (0, 0);
    let map = parts
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let chars: Vec<char> = line.chars().collect();

            if let Some(col) = chars.iter().position(|&c| c == '@') {
                start_pos = (row, col);
            }

            chars
        })
        .collect();
    let moves = parts
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| line.chars())
        .collect();

    (start_pos, map, moves)
}
