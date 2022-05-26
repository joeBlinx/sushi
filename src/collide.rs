use crate::types::Point;
pub trait Collider {
    fn get_collider(&self) -> Sphere;
}

pub struct Sphere {
    center: Point,
    radius: i32,
}
impl Sphere {
    pub fn new(center: Point, radius: i32) -> Sphere {
        Sphere { center, radius }
    }
}
pub fn collide<T: Collider, U: Collider>(collider1: &T, collider2: &U) -> bool {
    collide_sphere(&collider1.get_collider(), &collider2.get_collider())
}
fn collide_sphere(sphere1: &Sphere, sphere2: &Sphere) -> bool {
    let distance_x = sphere2.center.x - sphere1.center.x;
    let distance_x = distance_x * distance_x;

    let distance_y = sphere2.center.y - sphere1.center.y;
    let distance_y = distance_y * distance_y;

    let distance = ((distance_x + distance_y) as f32).sqrt();

    return distance < (sphere1.radius + sphere2.radius) as f32;
}
