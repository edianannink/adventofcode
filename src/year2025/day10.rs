use crate::year2025::examples;
use good_lp::{Solution, SolverModel, constraint, default_solver, variable, variables};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
struct Machine {
    lights: usize,
    buttons: Vec<usize>,
    joltage: Vec<usize>,
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

        let joltage = parts
            .last()
            .unwrap()
            .split(",")
            .map(|str| str.parse().unwrap())
            .collect::<Vec<usize>>();

        Self {
            lights,
            buttons,
            joltage,
        }
    }
}

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/year2025/input/day10.txt")
        .unwrap_or_else(|_| examples::DAY10.to_string());

    let machines: Vec<Machine> = input.lines().map(Machine::new).collect();

    solve(&machines)
}

fn solve(machines: &[Machine]) -> (String, String) {
    let mut part1 = 0;
    let mut part2 = 0;

    for machine in machines {
        let mut min_press = usize::MAX;

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((0, 0));
        visited.insert(0);

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

        part1 += min_press;
        part2 += solve_lp(machine);
    }

    (part1.to_string(), part2.to_string())
}

fn solve_lp(machine: &Machine) -> usize {
    let mut vars = variables!();
    let button_vars: Vec<_> = (0..machine.buttons.len())
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    let mut problem = vars
        .minimise(button_vars.iter().sum::<good_lp::Expression>())
        .using(default_solver);

    for (counter_idx, &joltage) in machine.joltage.iter().enumerate() {
        let expr = button_vars
            .iter()
            .enumerate()
            .filter(|(j, _)| (machine.buttons[*j] & (1 << counter_idx)) != 0)
            .map(|(_, &var)| var)
            .sum::<good_lp::Expression>();

        problem = problem.with(constraint!(expr == joltage as i32));
    }

    let solution = problem.solve().unwrap();
    button_vars
        .iter()
        .map(|&var| solution.value(var).round() as usize)
        .sum()
}
