use regex::Regex;
use std::ops::Mul;
const INPUT: &str = include_str!("../input.txt");
const REGEX: &str = r#"(mul\((\d+),(\d+)\))"#;

fn main() {
    let do_input = "do()".to_owned() + INPUT;
    let mut sum: u32 = mul_sum(&do_input);
    println!("The sum of the multiplication is: {}", sum);

    sum = do_mul(&do_input);
    println!("The sum of the do() multiplication is: {}", sum);
}

fn mul_sum(input_str: &str) -> u32 {
    Regex::new(REGEX)
        .unwrap()
        .captures_iter(input_str)
        .map(|mat| {
            mat.get(2)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap()
                .mul(mat.get(3).unwrap().as_str().parse::<u32>().unwrap())
        })
        .sum()
}

fn do_mul(input_str: &str) -> u32 {
    let mut sum = 0;
    let mut start_pos = 0;

    while let Some(start) = input_str[start_pos..].find("do()") {
        let start_index = start_pos + start + "do()".len();
        let end_index = input_str[start_index..]
            .find("don't()")
            .map_or(input_str.len(), |end| start_index + end);

        sum += mul_sum(&input_str[start_index..end_index]);
        start_pos = end_index;
    }
    sum
}
