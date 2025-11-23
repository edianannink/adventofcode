#![allow(dead_code)]

pub(crate) fn print_matrix<T: std::fmt::Display>(matrix: &Vec<Vec<T>>) {
    for line in matrix {
        for ch in line {
            print!("{ch}");
        }
        println!();
    }
    println!();
}
