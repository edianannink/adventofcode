use crate::year2025::examples;

pub fn solution() -> (usize, usize) {
    let input = std::fs::read_to_string("./src/year2025/input/day6.txt")
        .unwrap_or_else(|_| examples::DAY6.to_string());

    (part1(&input), part2(&input))
}

fn part1(input: &str) -> usize {
    let mut result = 0;

    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut columns: Vec<Vec<&str>> = vec![Vec::new(); rows[0].len()];
    for row in rows {
        for (i, &item) in row.iter().enumerate() {
            columns[i].push(item);
        }
    }

    for column in columns {
        let operator = *column.last().unwrap();
        let numbers: Vec<usize> = column
            .iter()
            .filter_map(|&number| number.parse::<usize>().ok())
            .collect();
        result += match operator {
            "+" => numbers.iter().sum::<usize>(),
            "*" => numbers.iter().product::<usize>(),
            _ => panic!("Unknown operator!"),
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let mut result = 0;

    let char_rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut char_columns: Vec<Vec<char>> = vec![Vec::new(); char_rows[0].len()];
    for char_row in char_rows {
        for (i, &item) in char_row.iter().enumerate() {
            char_columns[char_row.len() - 1 - i].push(item);
        }
    }

    for vecs in char_columns.split(|chars| chars.iter().all(|c| c.is_whitespace())) {
        let operator = vecs.last().unwrap().last().unwrap();
        let numbers: Vec<usize> = vecs
            .iter()
            .filter_map(|chars| {
                chars
                    .iter()
                    .filter(|&c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse()
                    .ok()
            })
            .collect();
        result += match operator {
            '+' => numbers.iter().sum::<usize>(),
            '*' => numbers.iter().product::<usize>(),
            _ => panic!("Unknown operator!"),
        }
    }

    result
}
