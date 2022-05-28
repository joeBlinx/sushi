use crate::types::Point;

pub struct Sphere {
    center: Point,
    radius: i32,
}

impl Sphere {
    pub fn new(center: Point, radius: i32) -> Sphere {
        Sphere { center, radius }
    }
    pub fn get_center(&self) -> Point {
        self.center.clone()
    }
    pub fn get_radius(&self) -> i32 {
        self.radius
    }
}
