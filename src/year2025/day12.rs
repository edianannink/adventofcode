use crate::year2025::examples;

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/year2025/input/day12.txt")
        .unwrap_or_else(|_| examples::DAY12.to_string());

    let part1 = input
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .filter(|tree| {
            let parts: Vec<_> = tree.split(": ").collect();
            let area: u32 = parts[0]
                .split('x')
                .map(|n| n.parse::<u32>().unwrap())
                .product();
            let total_shapes: u32 = parts[1]
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .sum();
            area >= 9 * total_shapes
        })
        .count()
        .to_string();

    (part1, String::new())
}
