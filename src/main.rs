use adventofcode::{year2024, year2025};

fn print_header(year: &str) {
    println!("===============================");
    println!("      Advent of Code {year}      ");
    println!("===============================");
}

fn print_day_solution(day: usize, part: (String, String)) {
    println!("Day {}:", day);
    println!("  Part 1: {}", part.0);
    println!("  Part 2: {}", part.1);
    println!("-------------------------------");
}

fn main() {
    print_header("2025");
    print_day_solution(1, year2025::day1::solution());
    print_day_solution(2, year2025::day2::solution());
    print_day_solution(3, year2025::day3::solution());
    print_day_solution(4, year2025::day4::solution());
    print_day_solution(5, year2025::day5::solution());
    print_day_solution(6, year2025::day6::solution());
    print_day_solution(7, year2025::day7::solution());
    print_day_solution(8, year2025::day8::solution());
    print_day_solution(9, year2025::day9::solution());
    print_day_solution(10, year2025::day10::solution());
    print_day_solution(11, year2025::day11::solution());
    print_day_solution(12, year2025::day12::solution());

    print_header("2024");
    print_day_solution(1, year2024::day1::solution());
    print_day_solution(2, year2024::day2::solution());
    print_day_solution(3, year2024::day3::solution());
    print_day_solution(4, year2024::day4::solution());
    print_day_solution(5, year2024::day5::solution());
    print_day_solution(6, year2024::day6::solution());
    print_day_solution(7, year2024::day7::solution());
    print_day_solution(8, year2024::day8::solution());
    print_day_solution(9, year2024::day9::solution());
    print_day_solution(10, year2024::day10::solution());
    print_day_solution(11, year2024::day11::solution());
    print_day_solution(12, year2024::day12::solution());
    print_day_solution(13, year2024::day13::solution());
    print_day_solution(14, year2024::day14::solution());
    print_day_solution(15, year2024::day15::solution());
    print_day_solution(16, year2024::day16::solution());
    print_day_solution(17, year2024::day17::solution());
}
