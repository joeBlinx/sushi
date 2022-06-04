use crate::collide::Collider;
use crate::entity::{EntityMovable, Movable};
use crate::shapes::Sphere;
use crate::types::{Color, GetDrawingRectangle, GetPosition, GetSize, Point, Size};
pub trait Power {
    fn use_power(&mut self);
    fn trigger_power(user: &impl GetPosition) -> Box<dyn PowerTrait>
    where
        Self: Sized;
    fn get_required_transo_truc() -> usize
    where
        Self: Sized;
}
pub trait PowerTrait: Collider<Sphere> + GetDrawingRectangle + Movable + Power {}
pub struct Sword {
    entity: EntityMovable,
}
impl Sword {
    pub fn new(x: i32, y: i32) -> Self {
        Sword {
            entity: EntityMovable::new(x, y, 10, 50, Color::YELLOW),
        }
    }
}
impl Power for Sword {
    fn use_power(&mut self) {
        println!("Attaaaaack!!!!");
    }
    fn trigger_power(user: &impl GetPosition) -> Box<dyn PowerTrait> {
        Box::new(Sword::new(user.get_x() + 50, user.get_y() - 10))
    }
    fn get_required_transo_truc() -> usize {
        3
    }
}
impl PowerTrait for Sword {}
impl Movable for Sword {
    fn move_xy(&mut self, x: i32, y: i32) {
        self.entity.move_xy(x, y);
    }
}
impl GetPosition for Sword {
    fn get_position(&self) -> Point {
        self.entity.get_position()
    }
}
impl GetSize for Sword {
    fn get_size(&self) -> Size {
        self.entity.get_size()
    }
}
impl GetDrawingRectangle for Sword {
    fn get_color(&self) -> Color {
        self.entity.get_color()
    }
}
impl Collider<Sphere> for Sword {
    fn get_collider(&self) -> Sphere {
        self.entity.get_collider()
    }
}
