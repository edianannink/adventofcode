const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut values = [Vec::new(), Vec::new()];
    for line in INPUT.lines() {
        let mut split = line.split_whitespace();
        values[0].push(split.next().unwrap().parse::<usize>().unwrap());
        values[1].push(split.next().unwrap().parse::<usize>().unwrap());
    }

    // Sort the values
    values.iter_mut().for_each(|v| v.sort());

    // Iterate over each vec member and calculate the difference
    println!(
        "The sum of the differences is: {}",
        sum_of_difference(&values)
    );

    // Iterate over each vec and calculate sum of similarity scores
    println!(
        "The sum of the similarity scores is: {}",
        sum_of_similarity(&values)
    );
}

fn sum_of_difference(values: &[Vec<usize>; 2]) -> usize {
    values[0]
        .iter()
        .zip(&values[1])
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn sum_of_similarity(values: &[Vec<usize>; 2]) -> usize {
    values[0]
        .iter()
        .map(|a| a * values[1].iter().filter(|&&b| b == *a).count())
        .sum()
}
