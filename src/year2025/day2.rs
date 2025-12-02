use crate::year2025::examples;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2025/input/day2.txt")
        .unwrap_or_else(|_| examples::DAY2.to_string());

    solve(&input)
}

fn solve(input: &str) -> (usize, usize) {
    let ranges: Vec<Vec<&str>> = input
        .split(',')
        .map(|range| range.split('-').collect())
        .collect();
    let mut part1 = 0;
    let mut part2 = 0;

    for range in ranges.iter() {
        let (first, second) = (
            range[0].trim().parse::<usize>().unwrap(),
            range[1].trim().parse::<usize>().unwrap(),
        );

        for id in first..=second {
            let id_str = id.to_string();
            let id_str_len = id_str.len();
            let id_str_len_mid = id_str_len >> 1;

            if id_str_len % 2 == 0
                && id_str[0..id_str_len_mid] == id_str[id_str_len_mid..id_str_len]
            {
                part1 += id;
            }

            'outer: for pattern_len in 1..=id_str_len_mid {
                if id_str_len % pattern_len != 0 {
                    continue;
                }

                let pattern = &id_str[0..pattern_len];
                for i in (pattern_len..id_str_len).step_by(pattern_len) {
                    if &id_str[i..i + pattern_len] != pattern {
                        continue 'outer;
                    }
                }

                part2 += id;
                break;
            }
        }
    }
    (part1, part2)
}
