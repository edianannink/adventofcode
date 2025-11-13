use crate::year2024::examples;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/input/day7.txt")
        .unwrap_or_else(|_| examples::DAY7.to_string());

    let mut day1 = 0;

    let data: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split(&[':', ' '])
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    for problem in data {
        if dfs(&problem) {
            day1 += problem[0];
        }
    }

    (day1, 0)
}

fn dfs(problem: &[usize]) -> bool {
    let mut stack = vec![(problem[1], 2)];

    while let Some((current_total, i)) = stack.pop() {
        if i >= problem.len() {
            if current_total == problem[0] {
                return true;
            }
            continue;
        }

        if current_total > problem[0] {
            continue;
        }

        stack.push((current_total + problem[i], i + 1));
        stack.push((current_total * problem[i], i + 1));
    }

    false
}
