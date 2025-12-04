use crate::year2025::examples;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2025/input/day3.txt")
        .unwrap_or_else(|_| examples::DAY3.to_string());

    (part1(&input), part2(&input))
}

fn part1(input: &str) -> usize {
    let mut result = 0;

    for line in input.lines() {
        let mut max = 0;
        let joltage: Vec<char> = line.chars().collect();
        for l in 0..joltage.len() {
            for r in (l + 1)..joltage.len() {
                let jolts = format!("{}{}", joltage[l], joltage[r])
                    .parse::<usize>()
                    .unwrap();
                if jolts > max {
                    max = jolts;
                }
            }
        }
        result += max;
    }

    result
}

fn part2(input: &str) -> usize {
    let mut result = 0;

    for line in input.lines() {
        let mut remaining: Vec<usize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        let mut selected = Vec::new();

        while selected.len() < 12 {
            let needed = 12 - selected.len();
            let search_until = remaining.len() - needed + 1;

            let max_digit = remaining[..search_until].iter().max().unwrap();
            let idx = remaining[..search_until]
                .iter()
                .position(|&d| d == *max_digit)
                .unwrap();

            selected.push(remaining[idx]);
            remaining = remaining[idx + 1..].to_vec();
        }

        result += selected.iter().fold(0, |acc, &d| acc * 10 + d);
    }

    result
}
