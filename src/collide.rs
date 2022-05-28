use crate::shapes::Sphere;
pub trait Collider<T> {
    fn get_collider(&self) -> T;
}
pub trait CollideImpl {
    fn collide_impl(shape1: &Self, shape2: &Self) -> bool;
}
impl CollideImpl for Sphere {
    fn collide_impl(sphere1: &Self, sphere2: &Self) -> bool {
        let distance_x = sphere2.get_center().x - sphere1.get_center().x;
        let distance_x = distance_x * distance_x;

        let distance_y = sphere2.get_center().y - sphere1.get_center().y;
        let distance_y = distance_y * distance_y;

        let distance = ((distance_x + distance_y) as f32).sqrt();

        return distance < (sphere1.get_radius() + sphere2.get_radius()) as f32;
    }
}
pub fn collide<T: CollideImpl>(shape1: &impl Collider<T>, shape2: &impl Collider<T>) -> bool {
    CollideImpl::collide_impl(&shape1.get_collider(), &shape2.get_collider())
}
