use adventofcode::year2024::{day1, day2, day3, day4, day5, day6, day7};

fn print_header() {
    println!("===============================");
    println!("      Advent of Code 2024      ");
    println!("===============================");
}

fn print_day_solution(day: usize, part: (usize, usize)) {
    println!("Day {}:", day);
    println!("  Part 1: {:?}", part.0);
    println!("  Part 2: {:?}", part.1);
    println!("-------------------------------");
}

fn print_footer() {
    println!("===============================");
}

fn main() {
    print_header();
    print_day_solution(1, day1::solution());
    print_day_solution(2, day2::solution());
    print_day_solution(3, day3::solution());
    print_day_solution(4, day4::solution());
    print_day_solution(5, day5::solution());
    print_day_solution(6, day6::solution());
    print_day_solution(7, day7::solution());
    print_footer();
}
