use crate::year2024::examples;

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/year2024/input/day18.txt")
        .unwrap_or_else(|_| examples::DAY17.to_string());

    let (registers, program) = parse_input(&input);
    (alu(&registers, &program), dfs(&program).unwrap())
}

fn dfs(program: &[usize]) -> Option<String> {
    let mut stack = vec![(0, program.len() - 1, 0)];

    while let Some((a, idx, next_candidate)) = stack.pop() {
        let goal = program[idx];

        if next_candidate > 7 {
            continue;
        }

        for candidate in next_candidate..8 {
            let test_a = (a << 3) | candidate;

            let registers = [test_a, 0, 0];
            let output_str = alu(&registers, program);

            let output: Vec<usize> = output_str
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();

            if output[0] == goal {
                if idx == 0 {
                    return Some(test_a.to_string());
                }

                stack.push((a, idx, candidate + 1));
                stack.push((test_a, idx - 1, 0));

                break;
            }
        }
    }

    None
}
fn alu(registers: &[usize], program: &[usize]) -> String {
    let mut pc = 0;
    let mut output: Vec<usize> = Vec::new();
    let mut registers = registers.to_owned();
    loop {
        if pc >= program.len() {
            return output
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",");
        }
        let opcode = program[pc];
        let operand = program[pc + 1];

        match opcode {
            0 => {
                match operand {
                    0..=3 => registers[0] /= 1 << operand,
                    4..=6 => registers[0] /= 1 << registers[operand - 4],
                    _ => panic!("Invalid operand!"),
                }
                pc += 2;
            }
            1 => {
                registers[1] ^= operand;
                pc += 2;
            }
            2 => {
                match operand {
                    0..=3 => registers[1] = operand % 8,
                    4..=6 => registers[1] = registers[operand - 4] % 8,
                    _ => panic!("Invalid operand!"),
                }
                pc += 2;
            }
            3 => {
                if registers[0] != 0 {
                    pc = operand * 2;
                } else {
                    pc += 2;
                }
            }
            4 => {
                registers[1] ^= registers[2];
                pc += 2;
            }
            5 => {
                match operand {
                    0..=3 => output.push(operand % 8),
                    4..=6 => output.push(registers[operand - 4] % 8),
                    _ => panic!("Invalid operand!"),
                }
                pc += 2;
            }
            6 => {
                match operand {
                    0..=3 => registers[1] = registers[0] / (1 << operand),
                    4..=6 => registers[1] = registers[0] / (1 << registers[operand - 4]),
                    _ => panic!("Invalid operand!"),
                }
                pc += 2;
            }
            7 => {
                match operand {
                    0..=3 => registers[2] = registers[0] / (1 << operand),
                    4..=6 => registers[2] = registers[0] / (1 << registers[operand - 4]),
                    _ => panic!("Invalid operand!"),
                }
                pc += 2;
            }
            _ => panic!("Invalid opcode!"),
        }
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut registers = Vec::new();
    let mut program = Vec::new();
    for line in input.lines() {
        if line.contains("Register") {
            registers.push(
                line.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            );
        } else if line.contains("Program") {
            program = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();
        }
    }
    (registers, program)
}
