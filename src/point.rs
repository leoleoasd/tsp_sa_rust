use rand::prelude::*;
use text_io::scan;

#[derive(Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        ((dx * dx + dy * dy) as f64).sqrt()
    }

    pub fn to_coord(&self) -> (i32, i32) {
        ((self.x * 10 + 10) as i32, (self.y * 10 + 10) as i32)
    }
}

pub fn read_points(shuffle: bool) -> Vec<Point> {
    let mut points = Vec::<Point>::new();
    let n: i32;
    scan!("{}", n);
    for _ in 0..n {
        let (x, y): (i64, i64);
        scan!("{} {}\n", x, y);
        // if x == 0 && y == 0 {
        //     break
        // }
        points.push(Point { x, y });
    }
    if shuffle {
        points.shuffle(&mut thread_rng());
    }
    points
}
