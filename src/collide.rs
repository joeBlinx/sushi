use crate::shapes::Sphere;
use crate::types::Point;
pub trait Collider {
    fn get_collider(&self) -> Sphere;
}

pub fn collide<T: Collider, U: Collider>(collider1: &T, collider2: &U) -> bool {
    collide_sphere(&collider1.get_collider(), &collider2.get_collider())
}
fn collide_sphere(sphere1: &Sphere, sphere2: &Sphere) -> bool {
    let distance_x = sphere2.get_center().x - sphere1.get_center().x;
    let distance_x = distance_x * distance_x;

    let distance_y = sphere2.get_center().y - sphere1.get_center().y;
    let distance_y = distance_y * distance_y;

    let distance = ((distance_x + distance_y) as f32).sqrt();

    return distance < (sphere1.get_radius() + sphere2.get_radius()) as f32;
}
