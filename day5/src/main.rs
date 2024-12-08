const INPUT: &str = include_str!("../input.txt");

fn main() {
    let (rules, pages) = INPUT.split_once("\n\n").unwrap();

    let rules: Vec<(usize, usize)> = rules
        .lines()
        .map(|rule| {
            let (a, b) = rule.split_once("|").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let pages: Vec<Vec<usize>> = pages
        .lines()
        .map(|page| page.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();

    let mut failed_pages = vec![false; pages.len()];

    for (previous, rule) in rules.iter() {
        for p in 0..pages.len() {
            if failed_pages[p] {
                continue;
            } else if let Some(rule_index) = pages[p].iter().position(|x| x == rule) {
                if pages[p][rule_index..].contains(previous) {
                    failed_pages[p] = true
                }
            }
        }
    }

    let sum: usize = pages
        .iter()
        .enumerate()
        .filter_map(|(index, pages_vec)| {
            if !failed_pages[index] {
                Some(pages_vec[pages_vec.len() / 2])
            } else {
                None
            }
        })
        .sum();

    println!("Sum of correct pages: {sum}");
}
