use crate::year2024::examples;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2024/input/day2.txt")
        .unwrap_or_else(|_| examples::DAY2.to_string());

    (
        count_safe_reports(&input, monotonic_series),
        count_safe_reports(&input, monotonic_series_dampener),
    )
}

fn count_safe_reports(input: &str, condition: fn(&[usize]) -> bool) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|vec| {
            condition(vec) ^ condition(&vec.iter().rev().copied().collect::<Vec<usize>>())
        })
        .count()
}
fn window_condition(window: &[usize]) -> bool {
    window[0] < window[1]
        && window[0].abs_diff(window[1]) <= 3
        && window[0].abs_diff(window[1]) >= 1
}

fn monotonic_series(series: &[usize]) -> bool {
    series.windows(2).all(window_condition)
}

fn monotonic_series_dampener(series: &[usize]) -> bool {
    if monotonic_series(series) {
        true
    } else {
        for i in 0..series.len() {
            let mut series = series.to_vec();
            series.remove(i);
            if monotonic_series(&series) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        let (part1, part2) = (2, 4);
        assert_eq!(count_safe_reports(examples::DAY2, monotonic_series), part1);
        assert_eq!(
            count_safe_reports(examples::DAY2, monotonic_series_dampener),
            part2
        );
    }
}
