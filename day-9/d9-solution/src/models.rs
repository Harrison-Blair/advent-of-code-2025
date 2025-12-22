pub struct Point {
  pub x: u64,
  pub y: u64,
}

pub fn area(p1: &Point, p2: &Point) -> u128 {
    let width = p1.x.abs_diff(p2.x) as u128 + 1;
    let height = p1.y.abs_diff(p2.y) as u128 + 1;

    width * height
}