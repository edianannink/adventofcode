const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut sum = count_safe_reports(monotonic_series);
    println!("The number of safe reports is: {sum}");

    sum = count_safe_reports(monotonic_series_dampener);
    println!("The number of safe reports with dampener is: {sum}");
}

fn count_safe_reports(condition: fn(&[usize]) -> bool) -> usize {
    INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|vec| {
            condition(vec)
                ^ condition(&vec.iter().rev().copied().collect::<Vec<usize>>())
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
