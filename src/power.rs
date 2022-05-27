use crate::collide::Collider;
use crate::entity::{EntityMovable, Movable};
use crate::shapes::Sphere;
use crate::types::{Color, GetColor, GetDrawingRectangle, GetPosition, GetSize, Point, Size};
pub trait UsablePower {
    fn use_power(&mut self);
}
pub trait PowerTrait: Collider + GetDrawingRectangle + GetColor + Movable + UsablePower {}
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
impl UsablePower for Sword {
    fn use_power(&mut self) {
        println!("Attaaaaack!!!!");
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
impl GetDrawingRectangle for Sword {}
impl GetColor for Sword {
    fn get_color(&self) -> Color {
        self.entity.get_color()
    }
}
impl Collider for Sword {
    fn get_collider(&self) -> Sphere {
        self.entity.get_collider()
    }
}
