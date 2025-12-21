#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

// String formatting for Coordinate
impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// Creates a new coordinate from an (x, y, z) set of u32 coordinates
impl Coordinate {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n]
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        // Path compression: point directly to the root
        let root = self.find(self.parent[i]);
        self.parent[i] = root;
        root
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        }
    }

    pub fn get_component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = Vec::new();
        for i in 0..self.parent.len() {
            // If a node is its own parent, it is the root of a circuit
            if self.parent[i] == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}
