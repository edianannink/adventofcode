use crate::util::{Edge, JunctionBox, UnionFind};
use crate::year2025::examples;
use std::collections::HashMap;

const CONNECTIONS: usize = 10;

pub fn solution() -> (String, String) {
    let input = std::fs::read_to_string("./src/year2025/input/day8.txt")
        .unwrap_or_else(|_| examples::DAY8.to_string());

    let boxes: Vec<JunctionBox> = input.lines().map(JunctionBox::new).collect();

    solve(&boxes)
}

fn solve(boxes: &[JunctionBox]) -> (String, String) {
    let size = boxes.len();

    let mut edges = Vec::new();
    for i in 0..size {
        for j in (i + 1)..size {
            let dx = boxes[j].x as isize - boxes[i].x as isize;
            let dy = boxes[j].y as isize - boxes[i].y as isize;
            let dz = boxes[j].z as isize - boxes[i].z as isize;
            let weight = (dx * dx + dy * dy + dz * dz) as usize;
            edges.push(Edge::new(i, j, weight))
        }
    }
    edges.sort_by_key(|k| k.weight);

    (
        part1(&edges, size).to_string(),
        part2(boxes, &edges, size).to_string(),
    )
}

fn part1(edges: &[Edge], size: usize) -> usize {
    let mut uf = UnionFind::new(size);
    for edge in edges.iter().take(CONNECTIONS) {
        uf.union(edge.u, edge.v);
    }

    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for node in 0..size {
        let root = uf.find(node);
        groups.entry(root).or_default().push(node);
    }

    let mut values: Vec<usize> = groups.clone().into_values().map(|v| v.len()).collect();
    values.sort();

    values.iter().rev().take(3).product::<usize>()
}

fn part2(boxes: &[JunctionBox], edges: &[Edge], size: usize) -> usize {
    let mut uf = UnionFind::new(size);
    let mut final_edge = &Edge::default();

    for edge in edges {
        if uf.union(edge.u, edge.v) && uf.fully_connected() {
            final_edge = edge;
            break;
        }
    }
    boxes[final_edge.u].x * boxes[final_edge.v].x
}
