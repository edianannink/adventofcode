use crate::year2024::examples;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/input/day9.txt")
        .unwrap_or_else(|_| examples::DAY9.to_string());
    let trimmed_input = input.trim();

    (
        disk_map_part1(trimmed_input) as usize,
        disk_map_part2(trimmed_input),
    )
}

fn disk_map_part2(input: &str) -> usize {
    let mut map: Vec<(Option<usize>, usize)> = input
        .chars()
        .enumerate()
        .map(|(id, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            if id % 2 == 0 {
                (Some(id / 2), size)
            } else {
                (None, size)
            }
        })
        .collect();

    for i in (0..map.len()).rev() {
        if map[i].0.is_some() {
            for space in 0..i {
                if map[space].0.is_none() && map[i].1 <= map[space].1 {
                    let file_size = map[i].1;
                    let space_size = map[space].1;

                    map[space] = map[i];
                    map[i] = (None, file_size);

                    if file_size < space_size {
                        map.insert(space + 1, (None, space_size - file_size));
                    }
                    break;
                }
            }
        }
    }

    let mut checksum = 0;
    let mut index = 0;

    for (id, size) in map {
        if let Some(id) = id {
            for _ in 0..size {
                checksum += index * id;
                index += 1;
            }
        } else {
            for _ in 0..size {
                index += 1;
            }
        }
    }
    checksum
}

fn disk_map_part1(input: &str) -> usize {
    let mut map = Vec::new();
    for (i, c) in input.chars().enumerate() {
        let nr = c.to_digit(10).unwrap();
        for _ in 0..nr {
            if i % 2 == 0 {
                map.push((i / 2).to_string());
            } else {
                map.push('.'.to_string());
            }
        }
    }

    let mut result = Vec::new();
    let num_digits = map
        .iter()
        .filter(|c| c.chars().all(|c| c.is_ascii_digit()))
        .count();

    for str in map.clone().iter() {
        if result.len() < num_digits {
            if !str.chars().all(|c| c.is_ascii_digit()) {
                while let Some(rev_str) = map.pop() {
                    if rev_str.chars().all(|c| c.is_ascii_digit()) {
                        result.push(rev_str);
                        break;
                    }
                }
            } else {
                result.push(str.clone());
            }
        }
    }

    result
        .iter()
        .enumerate()
        .map(|(idx, str)| str.parse::<usize>().unwrap() * idx)
        .sum()
}
