mod input;
mod models;

use crate::models::Point;

fn part_one(points: Vec<Point>) -> u128 {
    let mut max_size: u128 = 0;

    for i in 0..points.len() {
        let p1 = &points[i];
        for j in (i + 1)..points.len() {
            let p2 = &points[j];
            let area = models::area(p1, p2);
            if area > max_size {
                max_size = area;
            }
        }
    }
    max_size
}

fn main() {
    let file_names = ["example.txt", "input.txt"];

    let e_points = input::read_file(file_names[0]);
    println!("Read {} pairs from {} ...", e_points.len(), file_names[0]);

    let i_points = input::read_file(file_names[1]);
    println!("Read {} pairs from {} ...", i_points.len(), file_names[1]);

    let e1 = part_one(e_points);
    println!("[Part-1] Example soltuion : {} ...", e1);
    println!("Should be : {}...", 50);
    assert_eq!(e1, 50);

    let i1 = part_one(i_points);
    println!("[Part-1] Input solution : {} ...", i1);
}
