use crate::year2024::examples;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/input/day9.txt")
        .unwrap_or_else(|_| examples::DAY9.to_string());

    (disk_map(input.trim()) as usize, 0)
}

fn disk_map(input: &str) -> usize {
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
