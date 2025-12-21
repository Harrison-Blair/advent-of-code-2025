mod input;
mod models;

use crate::models::{Coordinate, UnionFind};

// First part of the day-8 solution
// Find each box's closest other box, then connect them, repeat until all connected
fn part_one(electrical_boxes: Vec<Coordinate>) {
    let mut uf = UnionFind::new(electrical_boxes.len());

    let mut edges = Vec::new();
    for i in 0..electrical_boxes.len() {
        for j in (i + 1)..electrical_boxes.len() {
            let b1 = &electrical_boxes[i];
            let b2 = &electrical_boxes[j];

            let dx = b1.x as i64 - b2.x as i64;
            let dy = b1.y as i64 - b2.y as i64;
            let dz = b1.z as i64 - b2.z as i64;
            let dist_sq = dx * dx + dy * dy + dz * dz;

            edges.push((dist_sq, i, j));
        }
    }

    edges.sort_unstable_by_key(|k| k.0);

    let limit = if electrical_boxes.len() < 100 { 10 } else { 1000 };
    for (_dist, i, j) in edges.iter().take(limit) {
        uf.union(*i, *j);
    }

    let mut sizes = uf.get_component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let result: usize = sizes.iter().take(3).product();
    println!("Result: {}", result);
}

fn main() {
    let file_name: &str = "input.txt";
    println!("Reading from '{}' file...", file_name);

    let electrical_boxes = input::read_file(file_name);

    println!(
        "Loaded {} e-boxes from '{}'...",
        electrical_boxes.len(),
        file_name
    );

    part_one(electrical_boxes);
}
