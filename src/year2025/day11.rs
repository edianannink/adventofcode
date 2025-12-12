use crate::year2025::examples;
use std::collections::{HashMap, HashSet};

pub fn solution() -> (String, String) {
    let input_part1 = std::fs::read_to_string("./src/year2025/input/day11.txt")
        .unwrap_or_else(|_| examples::DAY11_1.to_string());
    let input_part2 = std::fs::read_to_string("./src/year2025/input/day11.txt")
        .unwrap_or_else(|_| examples::DAY11_2.to_string());

    (
        part1(&parse_input(&input_part1)).to_string(),
        part2(&parse_input(&input_part2)).to_string(),
    )
}

fn part1(inputs: &HashMap<String, Vec<String>>) -> usize {
    let mut result = 0;
    let mut stack = vec![(inputs.get("you").unwrap().clone(), Vec::new())];

    while let Some((outputs, visited)) = stack.pop() {
        for output in outputs {
            if output == "out" {
                result += 1;
            } else if let Some(next_output) = inputs.get(&output)
                && !visited.contains(&output)
            {
                let mut new_visited = visited.clone();
                new_visited.push(output.clone());
                stack.push((next_output.clone(), new_visited));
            }
        }
    }

    result
}

fn part2(inputs: &HashMap<String, Vec<String>>) -> usize {
    let to_fft = count_paths(inputs, "svr", "fft", 10);
    let fft_to_dac = count_paths(inputs, "fft", "dac", 18);
    let dac_to_end = count_paths(inputs, "dac", "out", 10);

    to_fft * fft_to_dac * dac_to_end
}

fn count_paths(
    inputs: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    limit: usize,
) -> usize {
    let mut cache: HashMap<(String, usize), usize> = HashMap::new();
    memoized_dfs(inputs, start, end, limit, &mut HashSet::new(), &mut cache)
}

fn memoized_dfs(
    inputs: &HashMap<String, Vec<String>>,
    current: &str,
    end: &str,
    limit: usize,
    visited: &mut HashSet<String>,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if current == end {
        return 1;
    }

    if visited.len() >= limit {
        return 0;
    }

    let cache_key = (current.to_string(), visited.len());
    if let Some(&cached_result) = cache.get(&cache_key) {
        return cached_result;
    }

    visited.insert(current.to_string());

    let mut result = 0;
    if let Some(outputs) = inputs.get(current) {
        for output in outputs {
            if !visited.contains(output) {
                result += memoized_dfs(inputs, output, end, limit, visited, cache);
            }
        }
    }

    visited.remove(current);
    cache.insert(cache_key, result);

    result
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut inputs = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let device = parts[0].to_string();
        let outputs: Vec<String> = parts[1]
            .split_whitespace()
            .map(|str| str.trim().to_string())
            .collect();
        inputs.insert(device, outputs);
    }
    inputs
}
