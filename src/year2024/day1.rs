use crate::year2024::examples;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2024/input/day1.txt")
        .unwrap_or_else(|_| examples::DAY1.to_string());

    let values = process_input(input);

    (sum_of_difference(&values), sum_of_similarity(&values))
}

fn process_input(input: String) -> [Vec<usize>; 2] {
    let mut values = [Vec::new(), Vec::new()];
    for line in input.lines() {
        let mut split = line.split_whitespace();
        values[0].push(split.next().unwrap().parse::<usize>().unwrap());
        values[1].push(split.next().unwrap().parse::<usize>().unwrap());
    }

    // Sort the values
    values.iter_mut().for_each(|v| v.sort());
    values
}

fn sum_of_difference(values: &[Vec<usize>; 2]) -> usize {
    values[0]
        .iter()
        .zip(&values[1])
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn sum_of_similarity(values: &[Vec<usize>; 2]) -> usize {
    values[0]
        .iter()
        .map(|a| a * values[1].iter().filter(|&&b| b == *a).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let (part1, part2) = (11, 31);
        let vectors = process_input(examples::DAY1.to_string());
        assert_eq!(sum_of_difference(&vectors), part1);
        assert_eq!(sum_of_similarity(&vectors), part2);
    }
}
