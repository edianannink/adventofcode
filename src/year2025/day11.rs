use std::collections::{HashMap, VecDeque};
use crate::year2025::examples;

const START: &[&str] = &["you", "svr"];
const DAC: &str = "dac";
const FFT: &str = "fft";
const END: &str = "out";
pub fn solution() -> (String, String) {
    let input_part1 = std::fs::read_to_string("./src/year2025/input/day11.txt")
        .unwrap_or_else(|_| examples::DAY11_1.to_string());
    let input_part2 = std::fs::read_to_string("./src/year2025/input/day11.txt")
        .unwrap_or_else(|_| examples::DAY11_2.to_string());

    (part1(&parse_input_part1(&input_part1)).to_string(), part2(&parse_input_part2(&input_part2)).to_string())
}

fn part1(inputs: &HashMap<String, Vec<String>>) -> usize {
    let mut result = 0;

    let mut stack = Vec::new();
    let visited: Vec<String> = Vec::new();
    stack.push((inputs.get(START[0]).unwrap().clone(), visited));

    while let Some((outputs, visited)) = stack.pop() {
        if outputs.first().unwrap() == END  {
            result += 1;
            continue;
        }

        for output in outputs {
            if output != END && !visited.contains(&output) {
                let mut new_visited = visited.clone();
                new_visited.push(output.clone());
                stack.push((inputs.get(&output).unwrap().clone(), new_visited));
            }
        }
    }

    result
}


fn part2(inputs: &HashMap<String, Vec<String>>) -> usize {
    let mut result = 0;

    let mut queue = VecDeque::new();
    let visited: Vec<String> = Vec::new();
    let both_visited: Vec<bool> = vec![false; 2];
    queue.push_back((inputs.get(END).unwrap().clone(), visited, both_visited));

    while let Some((outputs, visited, both_visited)) = queue.pop_front() {
        if outputs.first().unwrap() == START[1] {
            continue;
        }
        if both_visited.iter().all(|&b| b)  {
            result += 1;
            println!("{visited:?}");
            continue;
        }

        for output in outputs {
            if !visited.contains(&output) {
                let mut new_both_visited = both_visited.clone();
                match output.as_str() {
                    DAC => new_both_visited[0] = true,
                    FFT => new_both_visited[1] = true,
                    _ => {}
                }
                let mut new_visited = visited.clone();
                new_visited.push(output.clone());
                queue.push_back((inputs.get(&output).unwrap().clone(), new_visited, new_both_visited.clone()));
            }
        }

    }

    result
}

fn parse_input_part1(input: &str) -> HashMap<String, Vec<String>> {
    let mut inputs = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let device = parts[0].to_string();
        let outputs: Vec<String> = parts[1].split_whitespace().map(|str| str.trim().to_string()).collect();
        inputs.insert(device, outputs);
    }
    inputs
}

fn parse_input_part2(input: &str) -> HashMap<String, Vec<String>> {
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let device = parts[0].to_string();
        let outputs: Vec<String> = parts[1].split_whitespace().map(|str| str.trim().to_string()).collect();
        for output in outputs {
            inputs.entry(output).or_default().push(device.clone());
        }
    }
    inputs
}