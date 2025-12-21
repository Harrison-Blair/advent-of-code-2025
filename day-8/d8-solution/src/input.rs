use std::fs;

use crate::models::Coordinate;

// Read from file @ file_name, returns list of coordinates
pub fn read_file(file_name: &str) -> Vec<Coordinate> {
    let mut electrical_boxes: Vec<Coordinate> = Vec::new();

    let text = fs::read_to_string(file_name).expect("Failed to read from file...");

    for line in text.lines() {
        let items: Vec<u32> = line
            .split(',')
            .map(|s| s.trim().parse().expect("Error parsing number"))
            .collect();

        electrical_boxes.push(Coordinate::new(items[0], items[1], items[2]));
    }

    electrical_boxes
}