use crate::year2024::examples;

const ROWS: usize = 103;
const COLS: usize = 101;
const SECONDS: usize = 100;

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    fn new(pos: (isize, isize), vel: (isize, isize)) -> Self {
        Self { pos, vel }
    }

    fn final_pos(self, rows: isize, cols: isize, seconds: isize) -> (usize, usize) {
        (
            (self.pos.0 + self.vel.0 * seconds).rem_euclid(rows) as usize,
            (self.pos.1 + self.vel.1 * seconds).rem_euclid(cols) as usize,
        )
    }
}

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/input/day14.txt")
        .unwrap_or_else(|_| examples::DAY14.to_string());

    let robots = parse_input(&input);

    (part1(&robots), part2(&robots))
}

fn part1(robots: &Vec<Robot>) -> usize {
    let mut quadrants = [0usize; 4];
    for robot in robots {
        let (r, c) = robot.final_pos(ROWS as isize, COLS as isize, SECONDS as isize);
        let mid_row = ROWS / 2;
        let mid_col = COLS / 2;
        if r != mid_row && c != mid_col {
            let qr = if r < mid_row { 0 } else { 1 };
            let qc = if c < mid_col { 0 } else { 1 };
            let quadrant = qr * 2 + qc;
            quadrants[quadrant] += 1;
        }
    }
    quadrants.iter().product()
}

fn part2(robots: &Vec<Robot>) -> usize {
    for seconds in 0..(1 << 14) {
        let mut floor = vec![0u8; ROWS * COLS];
        for robot in robots {
            let (r, c) = robot.final_pos(ROWS as isize, COLS as isize, seconds as isize);
            floor[r * COLS + c] += 1;
        }

        if floor.windows(10).any(|w| w.iter().all(|x| *x > 0)) {
            return seconds;
        };
    }
    0
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let nums: Vec<isize> = line
            .split_whitespace()
            .flat_map(|str| {
                str.split(',').map(|str| {
                    str.chars()
                        .filter(|char| char.is_ascii_digit() || *char == '-')
                        .collect::<String>()
                })
            })
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse::<isize>().ok())
            .collect();

        let pos = (nums[1], nums[0]);
        let vel = (nums[3], nums[2]);

        robots.push(Robot::new(pos, vel));
    }
    robots
}
