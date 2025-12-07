use crate::year2024::examples;

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/input/day7.txt")
        .unwrap_or_else(|_| examples::DAY7.to_string());

    let mut day1 = 0;
    let mut day2 = 0;

    let data: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(&[':', ' ']).filter(|s| !s.is_empty()).collect())
        .collect();

    for problem in data {
        if dfs(&problem, false) {
            day1 += problem[0].parse::<usize>().unwrap();
        }
        if dfs(&problem, true) {
            day2 += problem[0].parse::<usize>().unwrap();
        }
    }

    (day1.to_string(), day2.to_string())
}

fn dfs(problem: &[&str], concat_enable: bool) -> bool {
    let mut stack = vec![(problem[1].to_string(), 2)];

    while let Some((current_total, i)) = stack.pop() {
        if i >= problem.len() {
            if current_total == problem[0] {
                return true;
            }
            continue;
        }

        if current_total.parse::<usize>().unwrap() > problem[0].parse::<usize>().unwrap() {
            continue;
        }

        let sum = (current_total.parse::<usize>().unwrap() + problem[i].parse::<usize>().unwrap())
            .to_string();
        let product = (current_total.parse::<usize>().unwrap()
            * problem[i].parse::<usize>().unwrap())
        .to_string();
        let concat = format!("{}{}", current_total, problem[i]);

        stack.push((sum, i + 1));
        stack.push((product, i + 1));
        if concat_enable {
            stack.push((concat, i + 1));
        }
    }

    false
}
