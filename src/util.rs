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

#[derive(Debug)]
pub(crate) struct JunctionBox {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) z: usize,
}

impl JunctionBox {
    pub(crate) fn new(str: &str) -> Self {
        let coords: Vec<usize> = str
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Edge {
    pub(crate) u: usize,
    pub(crate) v: usize,
    pub(crate) weight: usize,
}

impl Edge {
    pub(crate) fn new(u: usize, v: usize, weight: usize) -> Self {
        Self { u, v, weight }
    }
}

#[derive(Debug)]
pub(crate) struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub(crate) fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    pub(crate) fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub(crate) fn union(&mut self, x: usize, y: usize) -> bool {
        let root_a = self.find(x);
        let root_b = self.find(y);
        if root_a != root_b {
            self.parent[root_b] = root_a;
            true
        } else {
            false
        }
    }

    pub(crate) fn fully_connected(&mut self) -> bool {
        let root = self.find(0);
        (1..self.parent.len()).all(|i| self.find(i) == root)
    }
}
