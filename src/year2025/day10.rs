use std::collections::{HashSet, VecDeque};

use crate::year2025::examples;

#[derive(Debug, Clone)]
struct Machine {
    lights: usize,
    buttons: Vec<usize>,
}

impl Machine {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line
            .split_whitespace()
            .map(|line| {
                let mut chars = line.chars();
                chars.next();
                chars.next_back();
                chars.as_str()
            })
            .collect();

        let mut lights = 0;
        for (i, c) in parts[0].chars().enumerate() {
            if c == '#' {
                lights |= 1 << i;
            }
        }

        let mut buttons: Vec<usize> = Vec::new();
        for s in parts.iter().skip(1).take(parts.len() - 2) {
            let mut schematic = 0;
            for d in s.chars().filter_map(|c| c.to_digit(10)) {
                schematic |= 1 << d;
            }
            buttons.push(schematic);
        }

        Self { lights, buttons }
    }
}

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/year2025/input/day10.txt")
        .unwrap_or_else(|_| examples::DAY10.to_string());

    let machines: Vec<Machine> = input.lines().map(Machine::new).collect();

    (solve(&machines).to_string(), String::from("Python"))
}

fn solve(machines: &[Machine]) -> usize {
    let mut result = 0;

    for machine in machines {
        let mut min_press = usize::MAX;

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((0, 0));

        while let Some((state, presses)) = queue.pop_front() {
            if state == machine.lights {
                min_press = min_press.min(presses);
                break;
            }

            for &button in &machine.buttons {
                let new_state = button ^ state;
                if visited.insert(new_state) {
                    queue.push_back((new_state, presses + 1));
                }
            }
        }

        result += min_press;
    }

    result
}
