use std::collections::HashMap;

use crate::year2025::examples;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Box {
    x: isize,
    y: isize,
    z: isize,
}

impl Box {
    fn new(coord_str: &str) -> Self {
        let coords: Vec<isize> = coord_str
            .split(",")
            .map(|b| b.parse::<isize>().unwrap())
            .collect();
        Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    fn euclidean_distance(&self, other: &Box) -> usize {
        let float = (((self.x - other.x) as f64).powi(2)
            + ((self.y - other.y) as f64).powi(2)
            + ((self.z - other.z) as f64).powi(2))
        .sqrt();
        float.round() as usize
    }
}

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/year2025/input/day9.txt")
        .unwrap_or_else(|_| examples::DAY8.to_string());

    let mut boxes: Vec<Box> = input.lines().map(Box::new).collect();

    for a in &boxes {
        let mut vec = Vec::new();
        for b in &boxes {
            vec.push((a.euclidean_distance(b), b));
        }
        vec.sort_by_key(|k| k.0);
        for line in vec {
            println!("{line:?}");
        }
        println!();
    }

    (String::new(), String::new())
}
