use crate::year2025::examples;

#[derive(Default, Debug)]
struct XmasTree {
    cols: usize,
    rows: usize,
    shapes: Vec<usize>,
}

impl XmasTree {
    fn new(s: &str) -> Self {
        let parts = s.split(':').collect::<Vec<_>>();
        let dimensions = parts[0]
            .split('x')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        Self {
            cols: dimensions[0],
            rows: dimensions[1],
            shapes: parts[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Shapes {
    shape: Vec<Vec<usize>>,
}

impl Shapes {
    fn new(s: &str) -> Self {
        Self {
            shape: s
                .lines()
                .skip(1)
                .map(|c| {
                    c.chars()
                        .filter_map(|c| match c {
                            '#' => Some(1),
                            '.' => Some(0),
                            _ => None,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn orientations(self) -> Vec<Self> {
        let rows = self.shape.len();
        let cols = self.shape[0].len();

        let original = self;

        // 90° clockwise: new[col][rows-1-row] = old[row][col]
        let rot_90 = Self {
            shape: (0..cols)
                .map(|new_row| {
                    (0..rows)
                        .rev()
                        .map(|old_row| original.shape[old_row][new_row])
                        .collect()
                })
                .collect(),
        };

        // 180°: new[rows-1-row][cols-1-col] = old[row][col]
        let rot_180 = Self {
            shape: original
                .shape
                .iter()
                .rev()
                .map(|row| row.iter().rev().copied().collect())
                .collect(),
        };

        // 270° clockwise: new[cols-1-col][row] = old[row][col]
        let rot_270 = Self {
            shape: (0..cols)
                .rev()
                .map(|new_row| {
                    (0..rows)
                        .map(|old_row| original.shape[old_row][new_row])
                        .collect()
                })
                .collect(),
        };

        // Horizontal flip: new[row][cols-1-col] = old[row][col]
        let flip_h = Self {
            shape: original
                .shape
                .iter()
                .map(|row| row.iter().rev().copied().collect())
                .collect(),
        };

        // Vertical flip: new[rows-1-row][col] = old[row][col]
        let flip_v = Self {
            shape: original.shape.iter().rev().cloned().collect(),
        };

        // Diagonal flip (transpose): new[col][row] = old[row][col]
        let flip_d1 = Self {
            shape: (0..cols)
                .map(|col| (0..rows).map(|row| original.shape[row][col]).collect())
                .collect(),
        };

        // Anti-diagonal flip: new[cols-1-col][rows-1-row] = old[row][col]
        let flip_d2 = Self {
            shape: (0..cols)
                .rev()
                .map(|col| {
                    (0..rows)
                        .rev()
                        .map(|row| original.shape[row][col])
                        .collect()
                })
                .collect(),
        };

        vec![
            original, rot_90, rot_180, rot_270, flip_h, flip_v, flip_d1, flip_d2,
        ]
    }
}

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/year2025/input/day12.txt")
        .unwrap_or_else(|_| examples::DAY12.to_string());

    let (shapes, xmas_trees) = parse_input(&input);

    (part1(&shapes, &xmas_trees).to_string(), String::new())
}

fn part1(shapes: &[Shapes], xmas_trees: &[XmasTree]) -> usize {
    let owned = shapes.clone().iter().next().unwrap().orientations();
    println!("{:?} {xmas_trees:?}", owned[0].orientations());
    0
}

fn parse_input(input: &str) -> (Vec<Shapes>, Vec<XmasTree>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();

    (
        parts
            .iter()
            .take(parts.len() - 1)
            .map(|&s| Shapes::new(s))
            .collect(),
        parts.last().unwrap().lines().map(XmasTree::new).collect(),
    )
}
