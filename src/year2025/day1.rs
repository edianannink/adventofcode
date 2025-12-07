use crate::year2025::examples;

const CLICKS: isize = 100;

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/year2025/input/day1.txt")
        .unwrap_or_else(|_| examples::DAY1.to_string());

    solve(&input)
}

fn solve(input: &str) -> (String, String) {
    let mut dial: isize = CLICKS / 2;
    let mut total_zeros = 0;
    let mut on_zero = 0;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let rotation = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        match direction {
            'R' => {
                let start = (dial + rotation).rem_euclid(CLICKS);
                for i in 1..rotation {
                    if (dial.rem_euclid(CLICKS) + i).rem_euclid(CLICKS) == 0 {
                        total_zeros += 1;
                    }
                }
                dial = start;
            }
            'L' => {
                let start = (dial - rotation).rem_euclid(CLICKS);
                for i in 1..rotation {
                    if (dial.rem_euclid(CLICKS) - i).rem_euclid(CLICKS) == 0 {
                        total_zeros += 1;
                    }
                }
                dial = start;
            }
            _ => {}
        }

        if dial == 0 {
            on_zero += 1;
        }
    }

    (on_zero.to_string(), (on_zero + total_zeros).to_string())
}
