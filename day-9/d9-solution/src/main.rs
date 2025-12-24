mod input;
mod models;

use crate::models::Point;

fn part_one(points: &Vec<Point>) -> u128 {
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

fn part_two(points: &Vec<Point>) -> u128 {
    let mut max_size: u128 = 0;

    for i in 0..points.len() {
        let p1 = &points[i];
        for j in (i + 1)..points.len() {
            let p2 = &points[j];
            let area = models::area(p1, p2);
            println!(
                "Considering Pairs : ({}, {}) & ({}, {}) with an area of : {}",
                p1.x, p1.y, p2.x, p2.y, area
            );
            // No need to check if already larger
            if area <= max_size {
                continue;
            }

            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            let mut is_empty = true;
            for k in 0..points.len() {
                if i == k || j == k {
                    continue;
                }

                let pk = &points[k];
                if pk.x > min_x && pk.x < max_x && pk.y > min_y && pk.y < max_y {
                    is_empty = false;
                    println!("Failed due to point : ({}, {})", pk.x, pk.y);
                    break;
                }
            }
            if is_empty {
                max_size = area;
                println!("SUCCESS!");
            }
        }
    }

    max_size
}

fn main() {
    let file_names = ["example.txt", "input.txt"];

    println!("\n--- {} ---", file_names[0]);
    let e_points = input::read_file(file_names[0]);
    println!("Read {} pairs from {} ...", e_points.len(), file_names[0]);

    let e1 = part_one(&e_points);
    println!("[P1] Should be : {} is {}!", 50, e1);
    assert_eq!(e1, 50);

    let e2 = part_two(&e_points);
    println!("[P2] Should be : {} is {}!", 24, e2);
    assert_eq!(e2, 24);

    println!("\n--- {} ---", file_names[1]);
    let i_points = input::read_file(file_names[1]);
    println!("Read {} pairs from {} ...", i_points.len(), file_names[1]);

    println!();
    let i1 = part_one(&i_points);
    println!("[P1] Solution : {} ...", i1);

    let i2 = part_two(&i_points);
    println!("[P2] Solution : {} ...", i2);
}
