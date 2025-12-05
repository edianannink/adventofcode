use crate::year2025::examples;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2025/input/day5.txt")
        .unwrap_or_else(|_| examples::DAY5.to_string());

    let (ranges, ingredients) = parse_ingredients(&input);
    solve(&ranges, &ingredients)
}

fn solve(ranges: &[(usize, usize)], ingredients: &[usize]) -> (usize, usize) {
    (
        ingredients
            .iter()
            .filter(|&&ingredient| {
                ranges
                    .iter()
                    .any(|&(start, end)| ingredient >= start && ingredient <= end)
            })
            .count(),
        unique_ingredients(ranges),
    )
}

fn unique_ingredients(ranges: &[(usize, usize)]) -> usize {
    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort();

    let mut total = 0;
    let mut current_start = sorted_ranges[0].0;
    let mut current_end = sorted_ranges[0].1;

    for &(start, end) in &sorted_ranges[1..] {
        if start <= current_end + 1 {
            current_end = current_end.max(end);
        } else {
            total += current_end - current_start + 1;
            current_start = start;
            current_end = end;
        }
    }

    total += current_end - current_start + 1;
    total
}

fn parse_ingredients(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in input.lines() {
        if let Some((start, end)) = line.split_once('-') {
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            ranges.push((start, end));
        } else if !line.is_empty() {
            let ingredient: usize = line.parse().unwrap();
            ingredients.push(ingredient);
        }
    }

    (ranges, ingredients)
}
