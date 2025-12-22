use std::fs;

use crate::models::Point;

pub fn read_file(file_name: &str) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    let text =
        fs::read_to_string(file_name).expect(&format!("Failed to read from file: {}", file_name));

    for line in text.lines() {
        let nums: Vec<u64> = line
            .split(',')
            .map(|s| s.trim().parse().expect("Failed to parse number..."))
            .collect();
        points.push(Point {
            x: nums[0],
            y: nums[1],
        })
    }

    points
}